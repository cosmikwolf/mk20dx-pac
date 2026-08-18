# SPI PCSIS / PUSHR PCS: Bitmask Fields Described as Two-Value Enums

## Summary

`SPI MCR[PCSIS]` (bits 21:16) and `SPI PUSHR[PCS]` (bits 21:16) are **bitmasks**.
Each bit controls one PCSx chip-select line. The vendor SVD describes both as
6-bit fields that carry a two-entry `enumeratedValues` list, `0` and `1`, because
the reference manual states those two values **per bit**:

> **PCSIS[4:0] — Peripheral Chip Select x Inactive State.** Determines the
> inactive state of PCSx. 0: the inactive state of PCSx is low. 1: the inactive
> state of PCSx is high.
> — K20 Sub-Family Reference Manual, ch45, SPIx_MCR field descriptions

> **PCS[5:0].** Select which PCS signals are to be asserted for the transfer.
> 0: Negate the PCS[x] signal. 1: Assert the PCS[x] signal.
> — K20 Sub-Family Reference Manual, ch45, SPIx_PUSHR field descriptions

A whole-field enum with two entries cannot express a mask over six lines.

## Impact

svd2rust turned each field into an enum type with two variants:

```rust
pub enum Pcsis { Low = 0, High = 1 }
pub type PcsisW<'a, REG> = crate::FieldWriter<'a, REG, 6, Pcsis>;
```

Two consequences:

1. **The writer cannot select more than one line.** `w.pcsis().high()` writes the
   value 1 into a 6-bit field. That sets PCS0 inactive-high and leaves PCS1-5
   inactive-low. It reads as "all lines high" but does not do that. To drive a
   real mask the caller had to fall back to `unsafe { w.pcsis().bits(0x3F) }`.
2. **The reader is lossy.** `variant()` returns `Option<Pcsis>`, and any value
   above 1 returns `None`. A board with PCS0 and PCS2 in use (mask `0b000101`)
   reads back as `None`.

An earlier patch (`6bf816d`) renamed the variants from `_0` / `_1` to
`Low` / `High` and `Negate` / `Assert`. The rename made the wrong model read as
deliberate, and it broke every caller that used the numeric names.

## Fix

`patches/common/spi/pcs_bitmask.yaml` (SPI0, both variants) and
`patches/mk20d7/spi/spi1_pcs_bitmask.yaml` (SPI1) delete each field and add it
back without `enumeratedValues`:

```yaml
SPI0:
  MCR:
    _delete: [PCSIS]
    _add:
      PCSIS:
        description: Peripheral Chip Select x Inactive State (one bit per PCSx; ...)
        bitOffset: 16
        bitWidth: 6
        access: read-write
        writeConstraint: [0, 63]
```

svdtools 0.3.19 has no key that removes an `enumeratedValues` list, so delete plus
add is the way to express this. The `writeConstraint` range covers the whole
6-bit field, so svd2rust marks the writer `Safe`:

```rust
pub type PcsisR = crate::FieldReader;
pub type PcsisW<'a, REG> = crate::FieldWriter<'a, REG, 6, u8, crate::Safe>;
```

Callers now write a mask with no `unsafe` block:

```rust
spi.mcr().write(|w| w.mstr().master().pcsis().set(0b00_0001));
spi.pushr().write(|w| unsafe { w.txdata().bits(data) }.pcs().set(0b00_0001));
```

## Field width note

The SVD and `reference/kinetis.h` both give PCSIS a width of 6 bits:

```c
#define SPI_MCR_PCSIS(n)   (((n) & 0x3F) << 16)
```

The reference manual register diagram shows `PCSIS[4:0]` (bits 20:16) and marks
bit 21 reserved. The Teensyduino SPI library is inconsistent for the same reason:
`SPI.cpp` writes `SPI_MCR_PCSIS(0x1F)` and `SPI.h` `beginTransaction()` writes
`SPI_MCR_PCSIS(0x3F)`. Its `cs_mask[]` table for SPI0 tops out at `0x10`, so only
PCS0-PCS4 are reachable on Teensy 3.2. Bit 21 is reserved, so a write of `0x3F`
is harmless. The width stays at 6 to match the header and the vendor SVD.

## Related

- `docs/SPI_SR_W1C_BUG.md` — the other SPI SVD defect in this repo.

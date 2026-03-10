# SPI SR Register: w1c Bits Missing from ONE_TO_MODIFY_FIELDS_BITMAP

## Summary

The SPI0 (and SPI1) Status Register (SR) contains six **write-1-to-clear (w1c)** flag bits, but the SVD does not annotate them with `<modifiedWriteValues>oneToClear</modifiedWriteValues>`. As a result, svd2rust does not set `ONE_TO_MODIFY_FIELDS_BITMAP` for `SrSpec`, and the generated `write()` method starts from `RESET_VALUE = 0x0200_0000` — which has the TFFF bit (bit 25) already set to 1. Any `sr().write()` call that intends to clear a single flag inadvertently clears TFFF as well, because writing 1 to a w1c bit clears it.

## Impact

This bug **stalls SPI DMA transfers**. The failure chain:

1. DMA is configured to transfer a framebuffer over SPI. It triggers on TFFF: when the hardware sets TFFF=1 ("TX FIFO has space"), DMA pushes the next byte.
2. An SPI byte completes → TCF (Transfer Complete Flag) fires → SPI0 ISR runs.
3. ISR calls `spi.sr().write(|w| w.tcf().complete())` to clear TCF.
4. `write()` starts from `RESET_VALUE (0x0200_0000)`, applies TCF=1. The written value has **both** TCF=1 and TFFF=1.
5. Both are w1c: hardware clears both flags. TFFF goes to 0 ("TX FIFO full").
6. DMA sees TFFF=0 → stops requesting transfers → **DMA stalls indefinitely**.

### Current workaround

The FLXS1 firmware masks the SPI0 interrupt during DMA flushes:

```rust
// boards/flxs1_v1/src/tasks/display.rs
cortex_m::peripheral::NVIC::mask(mk20dx_hal::pac::Interrupt::SPI0);
oled.flush(&mut *bus, &mut dma_ch);  // ~17ms DMA transfer
unsafe { cortex_m::peripheral::NVIC::unmask(mk20dx_hal::pac::Interrupt::SPI0); }
```

This prevents the ISR from firing during DMA, but means async SPI wakeups are blocked for the entire flush duration.

## Root Cause

### In the SVD

The `TFFF` field (and 5 other SR fields) lacks `<modifiedWriteValues>oneToClear</modifiedWriteValues>`:

```xml
<!-- svd/MK20D7.svd — current -->
<field>
  <name>TFFF</name>
  <description>Transmit FIFO Fill Flag</description>
  <bitOffset>25</bitOffset>
  <bitWidth>1</bitWidth>
  <access>read-write</access>    <!-- should also have modifiedWriteValues -->
</field>
```

### In the generated PAC

Without w1c annotation, svd2rust generates a plain `BitWriter` and does not include the bit in `ONE_TO_MODIFY_FIELDS_BITMAP`:

```rust
// mk20d7/src/spi0/sr.rs — current
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
    // ONE_TO_MODIFY_FIELDS_BITMAP defaults to 0x0000_0000
}

impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0x0200_0000;  // bit 25 (TFFF) = 1
}
```

The `write()` method uses: `RESET_VALUE & !ONE_TO_MODIFY_FIELDS_BITMAP | ZERO_TO_MODIFY_FIELDS_BITMAP` as its initial value. With `ONE_TO_MODIFY_FIELDS_BITMAP = 0`, the starting value is `0x0200_0000` — TFFF is already "armed" to be cleared.

## Affected Fields

All six w1c flag bits in SPI SR (K20 Sub-Family Reference Manual, Section 43.3.5):

| Bit | Name | Description | Reset Value | w1c? |
|-----|------|-------------|-------------|------|
| 17  | RFDF | Receive FIFO Drain Flag | 0 | Yes |
| 19  | RFOF | Receive FIFO Overflow Flag | 0 | Yes |
| 25  | TFFF | Transmit FIFO Fill Flag | 1 | Yes — **this is the problematic one** |
| 27  | TFUF | Transmit FIFO Underflow Flag | 0 | Yes |
| 28  | EOQF | End of Queue Flag | 0 | Yes |
| 31  | TCF  | Transfer Complete Flag | 0 | Yes |

TFFF is unique among these: it's the only one whose **reset value is 1**. The others reset to 0, so the bug only manifests for TFFF (writing 0 to a w1c bit is a no-op).

## Proposed Fix

### Option A: SVD patch (preferred)

Add `<modifiedWriteValues>oneToClear</modifiedWriteValues>` to all six w1c fields in the SVD. svd2rust will then:

1. Generate `BitWriter1C` instead of `BitWriter` (prevents writing 0 to clear — only 1 clears).
2. Set `ONE_TO_MODIFY_FIELDS_BITMAP` to include all w1c bits, so `write()` starts with those bits masked out (set to 0), preventing accidental clears.

Add to `patches/common/spi/sr_enums.yaml` (or a new `sr_w1c.yaml`):

```yaml
SPI0:
  SR:
    RFDF:
      _write:
        modifiedWriteValues: oneToClear
    RFOF:
      _write:
        modifiedWriteValues: oneToClear
    TFFF:
      _write:
        modifiedWriteValues: oneToClear
    TFUF:
      _write:
        modifiedWriteValues: oneToClear
    EOQF:
      _write:
        modifiedWriteValues: oneToClear
    TCF:
      _write:
        modifiedWriteValues: oneToClear
```

**Note**: The exact svdtools YAML syntax for `modifiedWriteValues` may differ — check the [svdtools documentation](https://github.com/rust-embedded/svdtools) for the correct patch format. The SVD element to add inside each `<field>` is:

```xml
<modifiedWriteValues>oneToClear</modifiedWriteValues>
```

### Expected generated output after fix

```rust
// BitWriter1C only allows writing 1 (to clear). Writing 0 is a no-op.
pub type TfffW<'a, REG> = crate::BitWriter1C<'a, REG, Tfff>;

impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
    // All w1c bits: 17, 19, 25, 27, 28, 31
    //   0x8000_0000 (TCF)
    // + 0x1000_0000 (EOQF)
    // + 0x0800_0000 (TFUF)
    // + 0x0200_0000 (TFFF)
    // + 0x0008_0000 (RFOF)
    // + 0x0002_0000 (RFDF)
    // = 0x9A0A_0000
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x9A0A_0000;
}
```

With this bitmap, `write()` starts from:
```
RESET_VALUE & !ONE_TO_MODIFY_FIELDS_BITMAP
= 0x0200_0000 & !0x9A0A_0000
= 0x0200_0000 & 0x65F5_FFFF
= 0x0000_0000
```

All w1c bits start as 0 in the writer. Only explicitly set bits get cleared. The ISR's `sr().write(|w| w.tcf().complete())` would write `0x8000_0000` — clearing TCF only, leaving TFFF untouched. DMA continues uninterrupted.

### Option B: HAL workaround (if SVD fix is not feasible)

Replace `sr().write()` with `sr().write_with_zero()` in the HAL's ISR handler. `write_with_zero()` starts from 0 instead of RESET_VALUE:

```rust
// mk20dx-hal/src/spi.rs — on_spi_interrupt()
// Before:
spi.sr().write(|w| w.tcf().complete());
// After:
unsafe { spi.sr().write_with_zero(|w| w.tcf().complete()); }
```

This works but is fragile — every call site must remember to use `write_with_zero`. Option A fixes it at the source.

## Verification

After applying the fix, the SPI0 ISR masking during DMA can be removed from the firmware:

```rust
// This masking would no longer be needed:
// cortex_m::peripheral::NVIC::mask(mk20dx_hal::pac::Interrupt::SPI0);
// oled.flush(...)
// cortex_m::peripheral::NVIC::unmask(mk20dx_hal::pac::Interrupt::SPI0);
```

To verify:
1. Rebuild PAC: `make -C mk20dx-pac`
2. Confirm `ONE_TO_MODIFY_FIELDS_BITMAP` is `0x9A0A_0000` in generated `sr.rs`
3. Confirm `TfffW` is `BitWriter1C` (not `BitWriter`)
4. Remove SPI0 masking from display DMA flush
5. Run firmware — display DMA should complete without stalling

## Other Registers

This same issue may affect other peripherals with w1c status registers where a flag's reset value is 1. A systematic audit of all status registers in the SVD would be worthwhile.

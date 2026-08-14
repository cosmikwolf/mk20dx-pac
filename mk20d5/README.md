# mk20d5-pac

Peripheral Access Crate for the NXP Kinetis **MK20DX128VLH5**, as used on the
[Teensy 3.0](https://www.pjrc.com/store/teensy3.html).

Generated with [svd2rust](https://github.com/rust-embedded/svd2rust) from the
NXP vendor SVD, with correctness and ergonomics patches applied via
[svdtools](https://github.com/rust-embedded/svdtools). The patch set, the bugs
it corrects, and the regeneration pipeline are documented in the
[repository README](https://github.com/cosmikwolf/mk20dx-pac).

| Chip | Board | Core | Flash | RAM | DMA channels |
|------|-------|------|-------|-----|--------------|
| MK20DX128VLH5 | Teensy 3.0 | Cortex-M4 @ 50 MHz | 128K | 16K | 4 |

The other part in this family is [`mk20d7-pac`](https://crates.io/crates/mk20d7-pac).

## Usage

```toml
[dependencies]
mk20d5-pac = { version = "0.1", features = ["rt", "critical-section"] }
cortex-m = { version = "0.7", features = ["critical-section-single-core"] }
cortex-m-rt = "0.7"
```

`rt` installs the vector table. `critical-section` is what provides
`Peripherals::take()`; a single-core Cortex-M project supplies the
implementation, which is what `critical-section-single-core` above is for.

You also need a `memory.x` describing the part:

```
MEMORY
{
    FLASH (rx)  : ORIGIN = 0x00000000, LENGTH = 128K
    RAM   (rwx) : ORIGIN = 0x1FFFE000, LENGTH = 16K
}
```

## Example

```rust
#![no_std]
#![no_main]

use panic_halt as _;
use mk20d5_pac as pac;

#[cortex_m_rt::entry]
fn main() -> ! {
    let peripherals = pac::Peripherals::take().unwrap();

    // Enable the clock gate for GPIO port C
    peripherals.sim.scgc5().modify(|_, w| w.portc().enabled());

    // Configure pin C5 as a GPIO output
    peripherals.portc.pcr(5).write(|w| w.mux().gpio());
    peripherals.ptc.pddr().modify(|r, w| unsafe { w.bits(r.bits() | (1 << 5)) });

    loop {}
}
```

Register access follows the svd2rust API: peripherals are lowercase fields on
`Peripherals`, and registers are method accessors — `sim.scgc5()`, not
`SIM.scgc5`.

## Higher-level alternative

For driver-level code — typed pin handles, UART/SPI/I2C, DMA — see
[`mk20dx-hal`](https://github.com/cosmikwolf/mk20dx-hal), which builds on this
crate and re-exports it as `mk20dx_hal::pac`.

## License

Licensed under either of Apache-2.0 or MIT at your option.

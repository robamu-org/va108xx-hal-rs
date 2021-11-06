#![no_std]
#![no_main]

use defmt_rtt as _;    // transport layer for defmt logs
use panic_probe as _;  // panicking behavior
use va108xx_hal as _;

#[defmt_test::tests]
mod tests {
    use defmt::assert;

    #[test]
    fn always_passes() {
        assert!(true);
    }
}

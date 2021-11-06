#![no_std]
#![no_main]

#[defmt_test::tests]
mod tests {
    use defmt::assert;

    #[test]
    fn always_passes() {
        assert!(true);
    }
}

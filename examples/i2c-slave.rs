#![no_main]
#![no_std]
use cortex_m_rt::entry;
use embedded_hal::blocking::i2c::SevenBitAddress;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
use cortex_m::interrupt::Mutex;
use va108xx_hal::{
    i2c::{I2cAddress, I2cSlave, I2cSpeed, SlaveConfig},
    pac::{self, interrupt, I2CB},
    prelude::*,
    timer::{default_ms_irq_handler, set_up_ms_timer, Delay},
};
use core::cell::RefCell;

const I2C_ADDRESS: u8 = 0b1111000;
// Create a buffer with six elements
//static BB: BBBuffer<24> = BBBuffer::new();
//static mut RECV_BUF: Mutex<RefCell<[u8; 24]> = Mutex::new(RefCell::new());
static SEND_BUF: [u8; 24] = [0; 24];
static mut RECV_BUF: [u8; 24] = [0; 24];
static I2C_SLAVE: Mutex<RefCell<Option<I2cSlave<I2CB, SevenBitAddress>>>> =
    Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("-- Vorago I2C Slave Example --");
    let mut dp = pac::Peripherals::take().unwrap();
    let tim0 = set_up_ms_timer(
        &mut dp.SYSCONFIG,
        &mut dp.IRQSEL,
        50.mhz().into(),
        dp.TIM0,
        interrupt::OC0,
    );
    let mut delay = Delay::new(tim0);
    unsafe {
        cortex_m::peripheral::NVIC::unmask(pac::Interrupt::OC0);
    }

    let slave_addr = I2cAddress::Regular(I2C_ADDRESS);
    let sl_cfg = SlaveConfig::new(slave_addr);
    let i2cb: I2cSlave<I2CB, SevenBitAddress> = I2cSlave::i2cb_slave(
        dp.I2CB,
        sl_cfg,
        50.mhz(),
        I2cSpeed::Regular100khz,
        Some(&mut dp.SYSCONFIG),
    ).unwrap();
    cortex_m::interrupt::free(|cs| I2C_SLAVE.borrow(cs).replace(Some(i2cb)));
    loop {
        delay.delay_ms(500);
    }
}

#[interrupt]
fn OC0() {
    default_ms_irq_handler();
}

// Interrupt handler for slave SPI
#[interrupt]
fn OC2() {
    cortex_m::interrupt::free(|cs| {
        let mut i2cb = I2C_SLAVE.borrow(cs).borrow_mut();
        //i2cb.as_mut().unwrap().slave_irq_handler(&mut RECV_BUF);
    });
}

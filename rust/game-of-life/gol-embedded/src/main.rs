#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embedded_time::rate::*;
use panic_halt as _;
use rp_pico::hal::prelude::*;
use rp_pico::hal::pac;
use rp_pico::hal;

use gol_rs::gol::*;
use core::str::FromStr;

use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
use ssd1306::{prelude::*, Ssd1306};

#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    //
    // The default is to generate a 125 MHz system clock
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let core = pac::CorePeripherals::take().unwrap();
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());


    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Configure two pins as being I²C, not GPIO
    let sda_pin = pins.gpio0.into_mode::<hal::gpio::FunctionI2C>();
    let scl_pin = pins.gpio1.into_mode::<hal::gpio::FunctionI2C>();

    // Create the I²C driver, using the two pre-configured pins. This will fail
    // at compile time if the pins are in the wrong mode, or if this I²C
    // peripheral isn't available on these pins!
    let i2c = hal::I2C::i2c0(
        pac.I2C0,
        sda_pin,
        scl_pin,
        400.kHz(),
        &mut pac.RESETS,
        clocks.peripheral_clock,
    );

    let rosc = hal::rosc::RingOscillator::new(pac.ROSC);
    let rosc = rosc.initialize();

    // Create the I²C display interface:
    let interface = ssd1306::I2CDisplayInterface::new(i2c);

    // Create a driver instance and initialize:
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    // game
    let mut game = GameOfLife::from_str(
        r#"#
                #
               ###
                #
                                          ##    ##     ###
                                          # #  # #    ###
                                            #  #
                                          # #  # #   ###
                                          ##    ##    ###

                
        "#).unwrap();
    reset_random(&mut game, &rosc);
    let mut count = 0;

    

    loop {
        if count > 100 {
            reset_random(&mut game, &rosc);
            count = 0;
        }
        count += 1;

        display.clear();
        for idr in 0..64 {
            for idc in 0..128 {
                let index = idr*COLUMNS + idc;
                let value = game.screen.get(index);
                let color = match value.as_deref() {
                    Some(true) => BinaryColor::On,
                    _ => BinaryColor::Off
                };
                Pixel(Point::new(idc as i32, idr as i32), color).draw(&mut display).unwrap();
            }
        }
        display.flush().unwrap();
        game.advance();
        //delay.delay_ms(500);
    }

}

fn reset_random(game: &mut GameOfLife, rosc: &hal::rosc::RingOscillator<hal::rosc::Enabled>) {
    for idr in 0..64 {
        for idc in 0..128 {
            let index = idr*COLUMNS + idc;
            game.screen.set(index, rosc.get_random_bit());
        }
    }
}

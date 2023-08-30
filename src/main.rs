#![warn(clippy::all)]

use embedded_hal::prelude::*;
use ftdi_embedded_hal as hal;

fn main() {
    println!("Hello, world!");

    let ftdi_hal = {
        let device = ftdi::find_by_vid_pid(0x0403, 0x6014)
            .interface(ftdi::Interface::A)
            .open()
            .expect("FTDI USB device not found.");
        hal::FtHal::init_default(device).expect("Unable to initialize FTDI USB device.")
    };

    let (mut i2c, delay) = (
        ftdi_hal.i2c().expect("Unable to find FTDI I2C bus."),
        hal::Delay::default(),
    );

    for i in 0..=127 {
        let mut readbuf: [u8; 1] = [0; 1];
        let result = i2c.read(i, &mut readbuf);
        if let Ok(d) = result {
            // Do whatever work you want to do with found devices
            println!("Device found at address: 0x{:X}", i);
        }
    }

    //let i2c = shared_bus::BusManagerSimple::new(i2c);
}

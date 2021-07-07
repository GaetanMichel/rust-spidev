use std::io;
use std::io::prelude::*;
use spidev::{Spidev, SpidevOptions, SpiModeFlags};

fn create_spi() -> io::Result<Spidev> {
    let mut spi = Spidev::open("/dev/spidev1.7")?;
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(20_000)
        .mode(SpiModeFlags::SPI_MODE_0)
        .build();
    spi.configure(&options)?;
    Ok(spi)
}



fn write_spi(spi: &mut Spidev) -> io::Result<()> {
    spi.write(&[0x01, 0x02, 0x03])?;
    Ok(())
}



fn main() {
    let mut spi = create_spi().unwrap();
    println!("{:?}", write_spi(&mut spi).unwrap());
}

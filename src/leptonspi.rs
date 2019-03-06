use std::io;
use std::io::prelude::*;
use spidev::{Spidev, SpidevOptions, SpidevTransfer, SPI_MODE_3};

pub struct LeptonSpi {
    spi_dev: Spidev,
}

impl LeptonSpi {
    pub fn new(num: u8) -> io::Result<LeptonSpi> {

        let mut spi_dev = Spidev::open(format!("/dev/spidev/0.{}", num))?;
        spi_dev.configure(SpidevOptions::new()
                                        .bits_per_word(8)
                                        .max_speed_hz(20_000_000)
                                        .mode(SPI_MODE_3))?;

        Ok(LeptonSpi {
            spi_dev
        })
    }

    pub fn read(&mut self, mut buffer: &mut [u8]) -> io::Result<usize> {
        Ok(self.spi_dev.read(&mut buffer)?)
    }
}

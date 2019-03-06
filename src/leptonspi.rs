use std::io;
use std::io::prelude::*;
use spidev::{Spidev, SpidevOptions, SPI_MODE_3};

/// An opaque SPI device handle
pub struct LeptonSpi {
    spi_dev: Spidev,
}

impl LeptonSpi {
    /// Create a new SPI handle at `/dev/spidev0.{num}`
    pub fn new(num: u8) -> io::Result<LeptonSpi> {
        let spi_path = format!("/dev/spidev0.{}", num);
        let mut spi_dev = Spidev::open(spi_path)?;

        spi_dev.configure(SpidevOptions::new()
                                        .bits_per_word(8)
                                        .max_speed_hz(20_000_000)
                                        .mode(SPI_MODE_3))?;
        Ok(LeptonSpi {
            spi_dev
        })
    }
}

impl Read for LeptonSpi {
    fn read(&mut self, mut buffer: &mut [u8]) -> io::Result<usize> {
        Ok(self.spi_dev.read(&mut buffer)?)
    }
}

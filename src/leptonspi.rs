use std::io;
use std::io::prelude::*;
use spidev::{Spidev, SpidevOptions, SPI_MODE_3};

// NOTE: this could be declared 'static' instead if we wanted
// it to be (unsafely) mutable
const DEFAULT_SPI_SPEED: u32 = 20_000_000; // Hz

/// An opaque SPI device handle
pub struct LeptonSpi {
    spi_dev: Spidev,
}

impl LeptonSpi {
    /// Create a new SPI handle at `/dev/spidev0.{num}`
    pub fn new(num: u8, spi_speed: impl Into<Option<u32>>) -> io::Result<LeptonSpi> {
        let spi_speed = spi_speed.into();
        let spi_path = format!("/dev/spidev0.{}", num);
        let mut spi_dev = Spidev::open(spi_path)?;

        spi_dev.configure(SpidevOptions::new()
                                        .bits_per_word(8)
                                        .max_speed_hz(spi_speed
                                                      .unwrap_or(DEFAULT_SPI_SPEED)
                                                      )
                                        .mode(SPI_MODE_3))?;
        Ok(LeptonSpi { spi_dev })
    }
}

impl Read for LeptonSpi {
    fn read(&mut self, mut buffer: &mut [u8]) -> io::Result<usize> {
        Ok(self.spi_dev.read(&mut buffer)?)
    }
}

/// The data sent over SPI
// #[derive(Debug)]
pub struct LeptonPacket {
    pub valid:     bool,
    pub segment_no: u8, // 3 bits
    pub packet_no: u16, // only lower 12 bits used
    pub crc16:     u16,
    pub data: Vec<u8> // TODO: this datatype is inelegant
}

impl std::convert::From<[u8; 164]> for LeptonPacket {
    fn from(buffer: [u8; 164]) -> LeptonPacket {
        let valid      = !((buffer[0] & 0x0F) == 0x0F);
        let segment_no = (buffer[0] >> 4) & 0b00000111;
        let packet_no  = ((buffer[0] as u16) << 4) | (buffer[1] as u16);
        let crc16      = ((buffer[2] as u16) << 8) | (buffer[3] as u16);
        let data       = buffer[4..164].to_vec();
        LeptonPacket {
            valid,
            segment_no,
            packet_no,
            crc16,
            data
        }
    }
}

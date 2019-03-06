// mod types

use super::bindings::*;

#[derive(Debug)]
#[repr(u32)]
pub enum ByteOrder {
    LSBFirst = LEP_BYTE_ORDER_T_LEP_LSB_FIRST,
    MSBFirst = LEP_BYTE_ORDER_T_LEP_MSB_FIRST,
}

#[derive(Debug)]
#[repr(u32)]
pub enum OperationState {
    Ready = LEP_OPERATION_STATE_LEP_READY,
    Busy = LEP_OPERATION_STATE_LEP_BUSY,
    Waiting = LEP_OPERATION_STATE_LEP_WAITING,
}

/// Radiometry enable/disable
#[derive(Debug)]
#[repr(u32)]
pub enum EnableState {
    Disabled = LEP_ENABLE_STATE_LEP_DISABLED,
    Enabled = LEP_ENABLE_STATE_LEP_ENABLED,
}

#[derive(Debug)]
#[repr(u32)]
pub enum OnState {
    Off = LEP_ON_STATE_LEP_OFF,
    On = LEP_ON_STATE_LEP_ON,
}

/// Only commands over TWI (I2C) are implemented.
///
/// TWI is identically equal to the I2C protocol, but the name isn't
/// copyrighted. Atmel also calls their implementation the same thing.
#[derive(Debug)]
#[repr(u32)]
pub enum PortType {
    CciTwi = LEP_CAMERA_PORT_E_TAG_LEP_CCI_TWI,
    CciSpi = LEP_CAMERA_PORT_E_TAG_LEP_CCI_SPI, // unimplemented I think
}

/// Default appears to be 400kHz
#[derive(Debug)]
#[repr(u32)]
pub enum TwiClockRate {
    Khz100 = LEP_TWI_CLOCK_RATE_T_TAG_LEP_TWI_CLOCK_100KHZ,
    Khz400 = LEP_TWI_CLOCK_RATE_T_TAG_LEP_TWI_CLOCK_400KHZ,
    Mhz1   = LEP_TWI_CLOCK_RATE_T_TAG_LEP_TWI_CLOCK_1MHZ,
}

#[derive(Debug)]
#[repr(u32)]
pub enum SpiClockRate {
    Mhz2  = LEP_SPI_CLOCK_RATE_T_TAG_LEP_SPI_CLOCK_2MHZ,
    Mhz10 = LEP_SPI_CLOCK_RATE_T_TAG_LEP_SPI_CLOCK_10MHZ,
    Mhz20 = LEP_SPI_CLOCK_RATE_T_TAG_LEP_SPI_CLOCK_20MHZ,
}

#[derive(Debug)]
#[repr(u32)]
pub enum PortTag {
    CciTwi = LEP_CAMERA_PORT_E_TAG_LEP_CCI_TWI, 
    CciSpi = LEP_CAMERA_PORT_E_TAG_LEP_CCI_SPI, 
    End    = LEP_CAMERA_PORT_E_TAG_LEP_END_CCI_PORTS, 
}

impl From<u32> for PortTag {
    fn from(tag: u32) -> Self {
        match tag {
            LEP_CAMERA_PORT_E_TAG_LEP_CCI_TWI => PortTag::CciTwi,
            LEP_CAMERA_PORT_E_TAG_LEP_CCI_SPI => PortTag::CciSpi,
            _ => panic!("Invalid Port Tag!"),
        }
    }
}

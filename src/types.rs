// mod types

use super::bindings::*;

#[repr(u32)]
pub enum ByteOrder {
    LSBFirst = LEP_BYTE_ORDER_T_LEP_LSB_FIRST,
    MSBFirst = LEP_BYTE_ORDER_T_LEP_MSB_FIRST,
}

#[repr(u32)]
pub enum OperationState {
    Ready = LEP_OPERATION_STATE_LEP_READY,
    Busy = LEP_OPERATION_STATE_LEP_BUSY,
    Waiting = LEP_OPERATION_STATE_LEP_WAITING,
}

#[repr(u32)]
pub enum EnableState {
    Disabled = LEP_ENABLE_STATE_LEP_DISABLED,
    Enabled = LEP_ENABLE_STATE_LEP_ENABLED,
}

#[repr(u32)]
pub enum OnState {
    Off = LEP_ON_STATE_LEP_OFF,
    On = LEP_ON_STATE_LEP_ON,
}

#[repr(u32)]
pub enum PortType {
    CciTwi = LEP_CAMERA_PORT_E_TAG_LEP_CCI_TWI,
    CciSpi = LEP_CAMERA_PORT_E_TAG_LEP_CCI_SPI, // unimplemented I think
}

#[repr(u32)]
pub enum TwiClockRate {
    Khz100 = LEP_TWI_CLOCK_RATE_T_TAG_LEP_TWI_CLOCK_100KHZ,
    Khz400 = LEP_TWI_CLOCK_RATE_T_TAG_LEP_TWI_CLOCK_400KHZ,
    Mhz1   = LEP_TWI_CLOCK_RATE_T_TAG_LEP_TWI_CLOCK_1MHZ,
}

#[repr(u32)]
pub enum SpiClockRate {
    Mhz2  = LEP_SPI_CLOCK_RATE_T_TAG_LEP_SPI_CLOCK_2MHZ,
    Mhz10 = LEP_SPI_CLOCK_RATE_T_TAG_LEP_SPI_CLOCK_10MHZ,
    Mhz20 = LEP_SPI_CLOCK_RATE_T_TAG_LEP_SPI_CLOCK_20MHZ,
}

#[repr(C)]
pub struct CameraPort {
    port_id:   u16,
    port_type: PortType,
    baud_rate: u16,
    dev_addr:  u8,
}


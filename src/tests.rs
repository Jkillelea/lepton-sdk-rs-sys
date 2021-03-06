// mod tests
use super::bindings::*;
use super::error_codes::LeptonResult;
use super::*;

#[test]
fn test_open_port_manually() {
    let result: LeptonResult;
    let port_id   = 1; // 1 for /dev/i2c-1, 0 for /dev/i2c-0
    let port_type = PortTag::CciTwi;
    let baud_rate = 400; // kHz

    let mut port = LEP_CAMERA_PORT_DESC_T_TAG {
        portID: 1,                        // /dev/i2c-1
        portType: PortTag::CciTwi as u32, // TWI
        portBaudRate: 400,                // kHz
        deviceAddress: 0x2A,
    };

    unsafe {
        result = LEP_OpenPort(port_id, 
                              port_type as u32, 
                              baud_rate, 
                              &mut port).into();
    }

    println!("{:#?}", port);
    assert_eq!(result, LeptonResult::Ok);
}

#[test]
fn test_open_port_api() {
    let mut port = CameraPortDescriptor::new(1);
    println!("{:#?}", port);
    assert_eq!(port.open(), LeptonResult::Ok);
}

#[test]
fn test_enable_radiometry() {
    let mut port = CameraPortDescriptor::new(1);
    println!("{:#?}", port);
    assert_eq!(port.open(), LeptonResult::Ok);
    assert_eq!(port.enable_radiometry(), LeptonResult::Ok);
}



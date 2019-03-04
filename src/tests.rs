// mod tests

use super::bindings::*;
use super::error_codes::LeptonResult;
use super::*;

#[test]
fn test_open_port_manually() {
    let port_id   = 1; // 1 for /dev/i2c-1, 0 for /dev/i2c-0
    let port_type = PortTag::CciTwi;
    let baud_rate = 400; // kHz
    let mut port: LEP_CAMERA_PORT_DESC_T_TAG = unsafe { std::mem::zeroed() };
    let result: LeptonResult;

    unsafe {
        result = LEP_OpenPort(port_id, port_type as u32, baud_rate, &mut port).into();
    }

    println!("{:#?}", port);

    assert_eq!(result, LeptonResult::Ok);
}

#[test]
fn test_open_port_api() {
    let (port, result) = CameraPortDescriptor::open(1);
    println!("{:#?}", port);
    assert_eq!(result, LeptonResult::Ok);
}

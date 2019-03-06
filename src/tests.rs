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
    for i in &[0, 1] {
        let mut port = CameraPortDescriptor::new(*i);
        println!("{:#?}", port);
        assert_eq!(port.open(), LeptonResult::Ok);
    }
}

#[test]
fn test_enable_radiometry() {
    for i in &[0, 1] {
        let mut port = CameraPortDescriptor::new(*i);
        println!("{:#?}", port);
        assert_eq!(port.open(), LeptonResult::Ok);
        assert_eq!(port.enable_radiometry(), LeptonResult::Ok);
    }
}



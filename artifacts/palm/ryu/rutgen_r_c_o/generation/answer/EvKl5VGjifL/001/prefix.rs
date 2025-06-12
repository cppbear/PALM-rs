// Answer 0

#[test]
fn test_write_to_ryu_buffer_zero() {
    let mut buffer = [0u8; 16];
    unsafe {
        let len = write_to_ryu_buffer(0.0f32, buffer.as_mut_ptr());
    }
}

#[test]
fn test_write_to_ryu_buffer_positive_small() {
    let mut buffer = [0u8; 16];
    unsafe {
        let len = write_to_ryu_buffer(1.5e-10f32, buffer.as_mut_ptr());
    }
}

#[test]
fn test_write_to_ryu_buffer_negative_small() {
    let mut buffer = [0u8; 16];
    unsafe {
        let len = write_to_ryu_buffer(-1.5e-10f32, buffer.as_mut_ptr());
    }
}

#[test]
fn test_write_to_ryu_buffer_positive_large() {
    let mut buffer = [0u8; 16];
    unsafe {
        let len = write_to_ryu_buffer(3.4028234e+38f32, buffer.as_mut_ptr());
    }
}

#[test]
fn test_write_to_ryu_buffer_negative_large() {
    let mut buffer = [0u8; 16];
    unsafe {
        let len = write_to_ryu_buffer(-3.4028234e+38f32, buffer.as_mut_ptr());
    }
}

#[test]
fn test_write_to_ryu_buffer_positive_normal() {
    let mut buffer = [0u8; 16];
    unsafe {
        let len = write_to_ryu_buffer(12345.6789f32, buffer.as_mut_ptr());
    }
}

#[test]
fn test_write_to_ryu_buffer_negative_normal() {
    let mut buffer = [0u8; 16];
    unsafe {
        let len = write_to_ryu_buffer(-12345.6789f32, buffer.as_mut_ptr());
    }
}

#[test]
fn test_write_to_ryu_buffer_positive_tiny() {
    let mut buffer = [0u8; 16];
    unsafe {
        let len = write_to_ryu_buffer(1e-37f32, buffer.as_mut_ptr());
    }
}

#[test]
fn test_write_to_ryu_buffer_negative_tiny() {
    let mut buffer = [0u8; 16];
    unsafe {
        let len = write_to_ryu_buffer(-1e-37f32, buffer.as_mut_ptr());
    }
}


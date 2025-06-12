// Answer 0

#[test]
fn test_write_to_ryu_buffer_normal() {
    let value: f32 = 123.456;
    let mut result: [u8; 20] = [0; 20];
    let len = unsafe { value.write_to_ryu_buffer(result.as_mut_ptr()) };
    let formatted = std::str::from_utf8(&result[..len]).unwrap();
    assert_eq!(formatted, "123.456"); // Adjust based on expected output
}

#[test]
fn test_write_to_ryu_buffer_zero() {
    let value: f32 = 0.0;
    let mut result: [u8; 20] = [0; 20];
    let len = unsafe { value.write_to_ryu_buffer(result.as_mut_ptr()) };
    let formatted = std::str::from_utf8(&result[..len]).unwrap();
    assert_eq!(formatted, "0.0"); // Adjust based on expected output
}

#[test]
fn test_write_to_ryu_buffer_negative() {
    let value: f32 = -987.65;
    let mut result: [u8; 20] = [0; 20];
    let len = unsafe { value.write_to_ryu_buffer(result.as_mut_ptr()) };
    let formatted = std::str::from_utf8(&result[..len]).unwrap();
    assert_eq!(formatted, "-987.65"); // Adjust based on expected output
}

#[test]
fn test_write_to_ryu_buffer_small() {
    let value: f32 = 0.00012345;
    let mut result: [u8; 20] = [0; 20];
    let len = unsafe { value.write_to_ryu_buffer(result.as_mut_ptr()) };
    let formatted = std::str::from_utf8(&result[..len]).unwrap();
    assert!(formatted.starts_with("0.")); // Adjust based on expected output
}

#[test]
fn test_write_to_ryu_buffer_nan() {
    let value: f32 = std::f32::NAN;
    let mut result: [u8; 20] = [0; 20];
    let len = unsafe { value.write_to_ryu_buffer(result.as_mut_ptr()) };
    let formatted = std::str::from_utf8(&result[..len]).unwrap();
    assert_eq!(formatted, "NaN"); // Adjust based on expected output 
}

#[test]
fn test_write_to_ryu_buffer_infinity() {
    let value: f32 = std::f32::INFINITY;
    let mut result: [u8; 20] = [0; 20];
    let len = unsafe { value.write_to_ryu_buffer(result.as_mut_ptr()) };
    let formatted = std::str::from_utf8(&result[..len]).unwrap();
    assert_eq!(formatted, "inf"); // Adjust based on expected output
}

#[test]
fn test_write_to_ryu_buffer_negative_infinity() {
    let value: f32 = std::f32::NEG_INFINITY;
    let mut result: [u8; 20] = [0; 20];
    let len = unsafe { value.write_to_ryu_buffer(result.as_mut_ptr()) };
    let formatted = std::str::from_utf8(&result[..len]).unwrap();
    assert_eq!(formatted, "-inf"); // Adjust based on expected output
}


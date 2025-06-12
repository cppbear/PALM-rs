// Answer 0

#[test]
fn test_write_exponent3_zero() {
    let mut buf = [0u8; 4];
    let result = unsafe { write_exponent3(0, buf.as_mut_ptr()) };
}

#[test]
fn test_write_exponent3_one() {
    let mut buf = [0u8; 4];
    let result = unsafe { write_exponent3(1, buf.as_mut_ptr()) };
}

#[test]
fn test_write_exponent3_two() {
    let mut buf = [0u8; 4];
    let result = unsafe { write_exponent3(2, buf.as_mut_ptr()) };
}

#[test]
fn test_write_exponent3_three() {
    let mut buf = [0u8; 4];
    let result = unsafe { write_exponent3(3, buf.as_mut_ptr()) };
}

#[test]
fn test_write_exponent3_four() {
    let mut buf = [0u8; 4];
    let result = unsafe { write_exponent3(4, buf.as_mut_ptr()) };
}

#[test]
fn test_write_exponent3_five() {
    let mut buf = [0u8; 4];
    let result = unsafe { write_exponent3(5, buf.as_mut_ptr()) };
}

#[test]
fn test_write_exponent3_six() {
    let mut buf = [0u8; 4];
    let result = unsafe { write_exponent3(6, buf.as_mut_ptr()) };
}

#[test]
fn test_write_exponent3_seven() {
    let mut buf = [0u8; 4];
    let result = unsafe { write_exponent3(7, buf.as_mut_ptr()) };
}

#[test]
fn test_write_exponent3_eight() {
    let mut buf = [0u8; 4];
    let result = unsafe { write_exponent3(8, buf.as_mut_ptr()) };
}

#[test]
fn test_write_exponent3_nine() {
    let mut buf = [0u8; 4];
    let result = unsafe { write_exponent3(9, buf.as_mut_ptr()) };
}


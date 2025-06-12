// Answer 0

#[test]
fn test_byte_0() {
    let _result = Byte::byte(0);
}

#[test]
fn test_byte_1() {
    let _result = Byte::byte(1);
}

#[test]
fn test_byte_127() {
    let _result = Byte::byte(127);
}

#[test]
fn test_byte_255() {
    let _result = Byte::byte(255);
}

#[test]
#[should_panic]
fn test_byte_out_of_bounds_negative() {
    let _result = Byte::byte(256);
}

#[test]
#[should_panic]
fn test_byte_out_of_bounds_positive() {
    let _result = Byte::byte(257);
}


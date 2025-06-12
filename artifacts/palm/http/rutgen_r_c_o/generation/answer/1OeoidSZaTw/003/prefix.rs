// Answer 0

#[test]
fn test_is_valid_b_equals_tab() {
    let b: u8 = b'\t';
    is_valid(b);
}

#[test]
fn test_is_valid_b_less_than_32() {
    let b: u8 = 0;
    is_valid(b);
}

#[test]
fn test_is_valid_b_range_1() {
    let b: u8 = 31;
    is_valid(b);
}

#[test]
fn test_is_valid_b_range_2() {
    let b: u8 = 32;
    is_valid(b);
}

#[test]
fn test_is_valid_b_greater_than_tab() {
    let b: u8 = 33;
    is_valid(b);
}

#[test]
fn test_is_valid_b_value_127() {
    let b: u8 = 127;
    is_valid(b);
}

#[test]
fn test_is_valid_b_value_128() {
    let b: u8 = 128;
    is_valid(b);
}

#[test]
fn test_is_valid_b_value_255() {
    let b: u8 = 255;
    is_valid(b);
}


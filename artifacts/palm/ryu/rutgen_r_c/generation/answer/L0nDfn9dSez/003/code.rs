// Answer 0

#[test]
#[should_panic]
fn test_multiple_of_power_of_2_zero_value() {
    let value: u64 = 0; 
    let p: u32 = 5; 
    multiple_of_power_of_2(value, p);
}

#[test]
fn test_multiple_of_power_of_2_non_zero_value() {
    let value: u64 = 8; // 2^3, should be a multiple of 2^3
    let p: u32 = 3;
    assert!(multiple_of_power_of_2(value, p));
}

#[test]
fn test_multiple_of_power_of_2_with_boundary_power() {
    let value: u64 = 16; // 2^4, should be a multiple of 2^4
    let p: u32 = 4;
    assert!(multiple_of_power_of_2(value, p));
}

#[test]
fn test_multiple_of_power_of_2_non_multiple() {
    let value: u64 = 10; // 10 is not a multiple of 2^3 (which is 8)
    let p: u32 = 3;
    assert!(!multiple_of_power_of_2(value, p));
}

#[test]
fn test_multiple_of_power_of_2_highest_power() {
    let value: u64 = 1 << 63; // should be a multiple of 2^63
    let p: u32 = 63;
    assert!(multiple_of_power_of_2(value, p));
}

#[test]
#[should_panic]
fn test_multiple_of_power_of_2_invalid_power() {
    let value: u64 = 1; // any non-zero value
    let p: u32 = 64; // out of bounds
    multiple_of_power_of_2(value, p);
}


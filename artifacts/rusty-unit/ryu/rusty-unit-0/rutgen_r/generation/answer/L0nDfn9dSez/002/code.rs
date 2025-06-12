// Answer 0

#[test]
#[should_panic]
fn test_multiple_of_power_of_2_zero_value() {
    let value: u64 = 0;
    let p: u32 = 5;
    multiple_of_power_of_2(value, p);
}

#[test]
#[should_panic]
fn test_multiple_of_power_of_2_out_of_bounds_p() {
    let value: u64 = 8;
    let p: u32 = 64; // p must be less than 64
    multiple_of_power_of_2(value, p);
}

#[test]
fn test_multiple_of_power_of_2_minimal_valid() {
    let value: u64 = 1; // minimum non-zero value
    let p: u32 = 1;
    assert!(multiple_of_power_of_2(value, p));
}

#[test]
fn test_multiple_of_power_of_2_multiple() {
    let value: u64 = 4; // 2^2
    let p: u32 = 2;    
    assert!(multiple_of_power_of_2(value, p));
}

#[test]
fn test_multiple_of_power_of_2_non_multiple() {
    let value: u64 = 6; // not a multiple of 2^2
    let p: u32 = 2;    
    assert!(!multiple_of_power_of_2(value, p));
}

#[test]
fn test_multiple_of_power_of_2_high_value() {
    let value: u64 = 16; // 2^4
    let p: u32 = 4;    
    assert!(multiple_of_power_of_2(value, p));
}

#[test]
fn test_multiple_of_power_of_2_boundary_value() {
    let value: u64 = 32; // 2^5
    let p: u32 = 5;    
    assert!(multiple_of_power_of_2(value, p));
}


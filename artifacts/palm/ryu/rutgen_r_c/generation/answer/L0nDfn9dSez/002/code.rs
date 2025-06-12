// Answer 0

#[test]
#[should_panic]
fn test_multiple_of_power_of_2_panic_value_zero() {
    let value: u64 = 0; 
    let p: u32 = 5; 
    multiple_of_power_of_2(value, p);
}

#[test]
#[should_panic]
fn test_multiple_of_power_of_2_panic_p_equal_64() {
    let value: u64 = 16; 
    let p: u32 = 64; 
    multiple_of_power_of_2(value, p);
}

#[test]
fn test_multiple_of_power_of_2_true() {
    let value: u64 = 16; 
    let p: u32 = 4; 
    assert!(multiple_of_power_of_2(value, p));
}

#[test]
fn test_multiple_of_power_of_2_false() {
    let value: u64 = 18; 
    let p: u32 = 4; 
    assert!(!multiple_of_power_of_2(value, p));
}

#[test]
fn test_multiple_of_power_of_2_boundary_value() {
    let value: u64 = 0b1111; 
    let p: u32 = 4; 
    assert!(multiple_of_power_of_2(value, p));
}

#[test]
fn test_multiple_of_power_of_2_edge_case() {
    let value: u64 = 1; 
    let p: u32 = 1; 
    assert!(!multiple_of_power_of_2(value, p));
}


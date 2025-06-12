// Answer 0

#[test]
fn test_write_varu32_above_threshold() {
    let mut data = Vec::new();
    let n: u32 = 0b1000_0000;
    write_varu32(&mut data, n);
}

#[test]
fn test_write_varu32_below_threshold() {
    let mut data = Vec::new();
    let n: u32 = 0b0111_1111;
    write_varu32(&mut data, n);
}

#[test]
fn test_write_varu32_exactly_at_threshold() {
    let mut data = Vec::new();
    let n: u32 = 0b1000_0000;
    write_varu32(&mut data, n);
}

#[test]
fn test_write_varu32_large_number() {
    let mut data = Vec::new();
    let n: u32 = 0b1111_1111_1111_1111_1111_1111_1111_1111; // A large number
    write_varu32(&mut data, n);
} 

#[test]
fn test_write_varu32_zero() {
    let mut data = Vec::new();
    let n: u32 = 0; // Testing with zero
    write_varu32(&mut data, n);
}


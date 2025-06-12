// Answer 0

#[test]
fn test_new_inclusive_u8_valid_range() {
    let low: u8 = 10;
    let high: u8 = 20;
    let result = Uniform::<u8>::new_inclusive(low, high);
}

#[test]
fn test_new_inclusive_u8_equal() {
    let low: u8 = 15;
    let high: u8 = 15;
    let result = Uniform::<u8>::new_inclusive(low, high);
}

#[should_panic]
fn test_new_inclusive_u8_empty_range() {
    let low: u8 = 30;
    let high: u8 = 20; 
    let result = Uniform::<u8>::new_inclusive(low, high);
}

#[test]
fn test_new_inclusive_u16_valid_range() {
    let low: u16 = 1000;
    let high: u16 = 2000;
    let result = Uniform::<u16>::new_inclusive(low, high);
}

#[test]
fn test_new_inclusive_u32_valid_range() {
    let low: u32 = 1000000;
    let high: u32 = 2000000;
    let result = Uniform::<u32>::new_inclusive(low, high);
}

#[test]
fn test_new_inclusive_u64_valid_range() {
    let low: u64 = 10000000000;
    let high: u64 = 20000000000;
    let result = Uniform::<u64>::new_inclusive(low, high);
}

#[test]
fn test_new_inclusive_u128_valid_range() {
    let low: u128 = 100000000000000000000;
    let high: u128 = 200000000000000000000;
    let result = Uniform::<u128>::new_inclusive(low, high);
}

#[test]
fn test_new_inclusive_usize_valid_range() {
    let low: usize = 10;
    let high: usize = 100;
    let result = Uniform::<usize>::new_inclusive(low, high);
}

#[should_panic]
fn test_new_inclusive_usize_empty_range() {
    let low: usize = 200;
    let high: usize = 100; 
    let result = Uniform::<usize>::new_inclusive(low, high);
}


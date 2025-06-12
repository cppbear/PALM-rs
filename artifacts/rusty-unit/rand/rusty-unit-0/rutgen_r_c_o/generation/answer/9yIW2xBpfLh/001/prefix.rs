// Answer 0

#[test]
fn test_random_range_float() {
    let _value: f32 = rand::random_range(0.0..=1e9);
}

#[test]
fn test_random_range_small_range() {
    let _value: usize = rand::random_range(..=10);
}

#[test]
fn test_random_range_integer_range() {
    let _value: i32 = rand::random_range(1..=100);
}

#[test]
fn test_random_range_usize() {
    let _value: usize = rand::random_range(0..=usize::MAX);
}

#[test]
fn test_random_range_positive_integer() {
    let _value: i32 = rand::random_range(1..=i32::MAX);
}

#[test]
fn test_random_range_large_range() {
    let _value: usize = rand::random_range(10..=1000);
}

#[test]
fn test_random_range_u32() {
    let _value: u32 = rand::random_range(0..=std::u32::MAX);
}


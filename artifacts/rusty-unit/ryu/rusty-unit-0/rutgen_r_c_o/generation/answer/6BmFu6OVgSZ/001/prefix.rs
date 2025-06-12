// Answer 0

#[test]
fn test_multiple_of_power_of_5_minimum_value() {
    let value: u64 = 1;
    let p: u32 = 0;
    multiple_of_power_of_5(value, p);
}

#[test]
fn test_multiple_of_power_of_5_edge_case() {
    let value: u64 = 3125; // 5^5
    let p: u32 = 5;
    multiple_of_power_of_5(value, p);
}

#[test]
fn test_multiple_of_power_of_5_exceed_power() {
    let value: u64 = 625; // 5^4
    let p: u32 = 5;
    multiple_of_power_of_5(value, p);
}

#[test]
fn test_multiple_of_power_of_5_high_value() {
    let value: u64 = 1 << 63; // Large value
    let p: u32 = 0;
    multiple_of_power_of_5(value, p);
}

#[test]
fn test_multiple_of_power_of_5_zero_power() {
    let value: u64 = 25; // 5^2
    let p: u32 = 0;
    multiple_of_power_of_5(value, p);
}

#[test]
fn test_multiple_of_power_of_5_five_power() {
    let value: u64 = 15625; // 5^6
    let p: u32 = 6;
    multiple_of_power_of_5(value, p);
}

#[test]
fn test_multiple_of_power_of_5_maximum_value() {
    let value: u64 = u64::MAX; // 2^64 - 1
    let p: u32 = 32;
    multiple_of_power_of_5(value, p);
}

#[test]
fn test_multiple_of_power_of_5_non_multiple() {
    let value: u64 = 14; // Not a power of 5
    let p: u32 = 0;
    multiple_of_power_of_5(value, p);
}

#[test]
fn test_multiple_of_power_of_5_p_higher_than_possible() {
    let value: u64 = 5; // 5^1
    let p: u32 = 10;
    multiple_of_power_of_5(value, p);
}


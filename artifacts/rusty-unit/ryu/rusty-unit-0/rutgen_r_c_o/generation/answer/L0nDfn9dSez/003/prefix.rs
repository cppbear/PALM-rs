// Answer 0

#[test]
#[should_panic]
fn test_multiple_of_power_of_2_zero_value() {
    multiple_of_power_of_2(0, 0);
}

#[test]
fn test_multiple_of_power_of_2_smallest_power() {
    multiple_of_power_of_2(1, 0);
}

#[test]
fn test_multiple_of_power_of_2_power_of_2() {
    multiple_of_power_of_2(4, 2);
}

#[test]
fn test_multiple_of_power_of_2_non_power_of_2() {
    multiple_of_power_of_2(6, 2);
}

#[test]
fn test_multiple_of_power_of_2_highest_power() {
    multiple_of_power_of_2(u64::MAX, 63);
}

#[test]
fn test_multiple_of_power_of_2_edge_case() {
    multiple_of_power_of_2(u64::MAX - 1, 63);
}

#[test]
fn test_multiple_of_power_of_2_mid_value() {
    multiple_of_power_of_2(16, 4);
}

#[test]
fn test_multiple_of_power_of_2_large_value() {
    multiple_of_power_of_2(1 << 60, 60);
}

#[test]
fn test_multiple_of_power_of_2_two_power() {
    multiple_of_power_of_2(2, 1);
}

#[test]
fn test_multiple_of_power_of_2_large_power() {
    multiple_of_power_of_2(8, 3);
}


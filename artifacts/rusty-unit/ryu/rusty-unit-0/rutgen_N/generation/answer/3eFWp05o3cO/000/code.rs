// Answer 0

#[test]
fn test_multiple_of_power_of_2_32_case_1() {
    assert!(multiple_of_power_of_2_32(0, 0));
}

#[test]
fn test_multiple_of_power_of_2_32_case_2() {
    assert!(multiple_of_power_of_2_32(8, 3)); // 8 is a multiple of 2^3
}

#[test]
fn test_multiple_of_power_of_2_32_case_3() {
    assert!(!multiple_of_power_of_2_32(8, 4)); // 8 is not a multiple of 2^4
}

#[test]
fn test_multiple_of_power_of_2_32_case_4() {
    assert!(multiple_of_power_of_2_32(15, 4)); // 15 is a multiple of 2^4 (15 & 0b1111 == 0)
}

#[test]
fn test_multiple_of_power_of_2_32_case_5() {
    assert!(!multiple_of_power_of_2_32(7, 3)); // 7 is not a multiple of 2^3
}

#[test]
fn test_multiple_of_power_of_2_32_case_6() {
    assert!(multiple_of_power_of_2_32(16, 5)); // 16 is a multiple of 2^5
}

#[test]
fn test_multiple_of_power_of_2_32_case_7() {
    assert!(!multiple_of_power_of_2_32(32, 6)); // 32 is not a multiple of 2^6
}

#[test]
fn test_multiple_of_power_of_2_32_case_8() {
    assert!(multiple_of_power_of_2_32(0xFFFFFFFF, 32)); // All bits set, effectively zero for any p
}


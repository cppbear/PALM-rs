// Answer 0

#[test]
fn test_range_inclusive_is_empty_when_start_greater_than_end() {
    use core::ops::RangeInclusive;

    let range: RangeInclusive<i32> = 5..=3;
    assert!(range.is_empty());
}

#[test]
fn test_range_inclusive_is_empty_when_start_equal_to_end() {
    use core::ops::RangeInclusive;

    let range: RangeInclusive<i32> = 3..=3;
    assert!(!range.is_empty());
}

#[test]
fn test_range_inclusive_is_empty_when_start_less_than_end() {
    use core::ops::RangeInclusive;

    let range: RangeInclusive<i32> = 2..=5;
    assert!(!range.is_empty());
}

#[test]
fn test_range_to_inclusive_is_empty_when_end_zero() {
    use core::ops::RangeToInclusive;

    let range: RangeToInclusive<u8> = 0..=0;
    assert!(!range.is_empty());
}

#[test]
fn test_range_to_inclusive_is_empty_when_end_negative() {
    use core::ops::RangeToInclusive;

    let range: RangeToInclusive<i32> = -1..=0;
    assert!(range.is_empty());
}

#[test]
fn test_range_to_is_empty_when_end_zero() {
    use core::ops::RangeTo;

    let range: RangeTo<u8> = 0..;
    assert!(range.is_empty());
}

#[test]
fn test_range_to_is_empty_when_end_not_zero() {
    use core::ops::RangeTo;

    let range: RangeTo<u8> = 5..;
    assert!(range.is_empty());
}


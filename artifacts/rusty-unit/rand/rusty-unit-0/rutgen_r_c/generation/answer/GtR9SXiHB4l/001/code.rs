// Answer 0

#[test]
fn test_range_empty() {
    let range: core::ops::Range<u32> = 5..5;
    assert!(range.is_empty());
}

#[test]
fn test_range_non_empty() {
    let range: core::ops::Range<u32> = 1..10;
    assert!(!range.is_empty());
}

#[test]
fn test_range_inclusive_empty() {
    let range: core::ops::RangeInclusive<u32> = 5..=5;
    assert!(!range.is_empty());
}

#[test]
fn test_range_inclusive_non_empty() {
    let range: core::ops::RangeInclusive<u32> = 2..=8;
    assert!(!range.is_empty());
}

#[test]
fn test_range_to_empty() {
    let range: core::ops::RangeTo<u32> = ..5;
    assert!(range.is_empty());
}

#[test]
fn test_range_to_non_empty() {
    let range: core::ops::RangeTo<u32> = ..10;
    assert!(range.is_empty());
}

#[test]
fn test_range_to_inclusive_empty() {
    let range: core::ops::RangeToInclusive<u32> = ..=5;
    assert!(!range.is_empty());
}

#[test]
fn test_range_to_inclusive_non_empty() {
    let range: core::ops::RangeToInclusive<u32> = ..=10;
    assert!(!range.is_empty());
}


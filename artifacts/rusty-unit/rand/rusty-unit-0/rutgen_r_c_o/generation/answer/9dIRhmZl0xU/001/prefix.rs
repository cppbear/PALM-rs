// Answer 0

#[test]
fn test_range_inclusive_empty() {
    let range: RangeInclusive<u8> = 1..=0;
    let _ = range.is_empty();
}

#[test]
fn test_range_to_empty() {
    let range: RangeTo<u16> = ..0;
    let _ = range.is_empty();
}

#[test]
fn test_range_to_inclusive_empty() {
    let range: RangeToInclusive<u32> = ..=0;
    let _ = range.is_empty();
}

#[test]
fn test_range_inclusive_non_empty() {
    let range: RangeInclusive<u64> = 0..=1;
    let _ = range.is_empty();
}

#[test]
fn test_range_to_non_empty() {
    let range: RangeTo<u128> = ..1;
    let _ = range.is_empty();
}

#[test]
fn test_range_to_inclusive_non_empty() {
    let range: RangeToInclusive<usize> = ..=1;
    let _ = range.is_empty();
}


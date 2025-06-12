// Answer 0

#[test]
fn test_num_chars_empty_ranges() {
    let inst_ranges = InstRanges {
        goto: 0,
        ranges: vec![],
    };
    assert_eq!(inst_ranges.num_chars(), 0);
}

#[test]
fn test_num_chars_single_range() {
    let inst_ranges = InstRanges {
        goto: 1,
        ranges: vec![('a', 'a')],
    };
    assert_eq!(inst_ranges.num_chars(), 1);
}

#[test]
fn test_num_chars_multiple_non_overlapping_ranges() {
    let inst_ranges = InstRanges {
        goto: 2,
        ranges: vec![('a', 'c'), ('e', 'g')],
    };
    assert_eq!(inst_ranges.num_chars(), 7); // 'a', 'b', 'c', 'e', 'f', 'g'
}

#[test]
fn test_num_chars_overlapping_ranges() {
    let inst_ranges = InstRanges {
        goto: 3,
        ranges: vec![('a', 'c'), ('b', 'f')],
    };
    assert_eq!(inst_ranges.num_chars(), 6); // 'a', 'b', 'c', 'd', 'e', 'f'
}

#[test]
fn test_num_chars_large_ranges() {
    let inst_ranges = InstRanges {
        goto: 4,
        ranges: vec![('a', 'z'), ('A', 'Z')],
    };
    assert_eq!(inst_ranges.num_chars(), 52); // 26 lowercase + 26 uppercase
}

#[test]
fn test_num_chars_range_with_no_distinct_chars() {
    let inst_ranges = InstRanges {
        goto: 5,
        ranges: vec![('d', 'b')],
    };
    assert_eq!(inst_ranges.num_chars(), 0); // No valid range
}


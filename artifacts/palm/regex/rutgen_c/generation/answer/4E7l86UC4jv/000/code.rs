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
        goto: 0,
        ranges: vec![('a', 'a')],
    };
    assert_eq!(inst_ranges.num_chars(), 1);
}

#[test]
fn test_num_chars_multiple_disjoint_ranges() {
    let inst_ranges = InstRanges {
        goto: 0,
        ranges: vec![('a', 'c'), ('e', 'g')],
    };
    assert_eq!(inst_ranges.num_chars(), 7); // a, b, c, e, f, g
}

#[test]
fn test_num_chars_overlapping_ranges() {
    let inst_ranges = InstRanges {
        goto: 0,
        ranges: vec![('a', 'c'), ('b', 'd')],
    };
    assert_eq!(inst_ranges.num_chars(), 4); // a, b, c, d
}

#[test]
fn test_num_chars_range_with_non_consecutive_letters() {
    let inst_ranges = InstRanges {
        goto: 0,
        ranges: vec![('a', 'b'), ('d', 'f')],
    };
    assert_eq!(inst_ranges.num_chars(), 5); // a, b, d, e, f
}


// Answer 0

#[test]
fn test_matches_with_char_equal_to_range_start() {
    let inst_ranges = InstRanges {
        goto: 1,
        ranges: vec![
            ('a', 'c'),
            ('d', 'f'),
            ('g', 'j'),
            ('k', 'm'),
        ],
    };
    let char_input = Char('a' as u32);
    assert!(inst_ranges.matches(char_input));
}

#[test]
fn test_matches_with_char_equal_to_range_end() {
    let inst_ranges = InstRanges {
        goto: 1,
        ranges: vec![
            ('a', 'c'),
            ('d', 'f'),
            ('g', 'j'),
            ('k', 'm'),
        ],
    };
    let char_input = Char('c' as u32);
    assert!(inst_ranges.matches(char_input));
}

#[test]
fn test_matches_with_char_in_range() {
    let inst_ranges = InstRanges {
        goto: 1,
        ranges: vec![
            ('a', 'c'),
            ('d', 'f'),
            ('g', 'j'),
            ('k', 'm'),
        ],
    };
    let char_input = Char('b' as u32);
    assert!(inst_ranges.matches(char_input));
}


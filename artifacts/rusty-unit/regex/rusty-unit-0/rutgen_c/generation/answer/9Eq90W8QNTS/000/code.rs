// Answer 0

#[test]
fn test_matches_within_range() {
    let inst_ranges = InstRanges {
        goto: 1,
        ranges: vec![('a', 'z'), ('A', 'Z')],
    };
    let char_a = Char(97); // 'a'
    let char_A = Char(65); // 'A'
    let char_0 = Char(48); // '0'
    assert!(inst_ranges.matches(char_a));
    assert!(inst_ranges.matches(char_A));
    assert!(!inst_ranges.matches(char_0));
}

#[test]
fn test_matches_edge_cases() {
    let inst_ranges = InstRanges {
        goto: 1,
        ranges: vec![('a', 'c'), ('x', 'z')],
    };
    let char_a = Char(97); // 'a'
    let char_b = Char(98); // 'b'
    let char_c = Char(99); // 'c'
    let char_x = Char(120); // 'x'
    let char_y = Char(121); // 'y'
    let char_z = Char(122); // 'z'

    assert!(inst_ranges.matches(char_a));
    assert!(inst_ranges.matches(char_b));
    assert!(inst_ranges.matches(char_c));
    assert!(!inst_ranges.matches(char_x));
    assert!(!inst_ranges.matches(char_y));
    assert!(!inst_ranges.matches(char_z));
}

#[test]
fn test_matches_with_empty_ranges() {
    let inst_ranges = InstRanges {
        goto: 1,
        ranges: vec![],
    };
    let char_a = Char(97); // 'a'
    assert!(!inst_ranges.matches(char_a));
}


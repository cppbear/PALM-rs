// Answer 0

#[test]
fn test_matches_empty_ranges() {
    let instructions = InstRanges {
        goto: 0,
        ranges: vec![],
    };
    let c = Char(97); // 'a'
    assert_eq!(instructions.matches(c), false);
}

#[test]
fn test_matches_no_matching_ranges() {
    let instructions = InstRanges {
        goto: 0,
        ranges: vec![(Char(98), Char(100))], // range 'b' to 'd'
    };
    let c = Char(97); // 'a', not in range
    assert_eq!(instructions.matches(c), false);
}

#[test]
fn test_matches_first_range() {
    let instructions = InstRanges {
        goto: 0,
        ranges: vec![(Char(97), Char(98))], // range 'a' to 'b'
    };
    let c = Char(97); // 'a', matches first range
    assert_eq!(instructions.matches(c), true);
}

#[test]
fn test_matches_last_range() {
    let instructions = InstRanges {
        goto: 0,
        ranges: vec![(Char(97), Char(99)), (Char(100), Char(102))], // ranges 'a' to 'c' and 'd' to 'f'
    };
    let c = Char(102); // 'f', matches last range
    assert_eq!(instructions.matches(c), true);
}

#[test]
fn test_matches_non_matching_later_case() {
    let instructions = InstRanges {
        goto: 0,
        ranges: vec![(Char(97), Char(98)), (Char(100), Char(102))], // ranges 'a' to 'b' and 'd' to 'f'
    };
    let c = Char(99); // 'c', does not match any range
    assert_eq!(instructions.matches(c), false);
}

#[test]
fn test_matches_multiple_ranges() {
    let instructions = InstRanges {
        goto: 0,
        ranges: vec![(Char(97), Char(98)), (Char(99), Char(100)), (Char(101), Char(102))], // ranges 'a' to 'b', 'c' to 'd', 'e' to 'f'
    };
    let c = Char(101); // 'e', matches range 'e' to 'f'
    assert_eq!(instructions.matches(c), true);
}


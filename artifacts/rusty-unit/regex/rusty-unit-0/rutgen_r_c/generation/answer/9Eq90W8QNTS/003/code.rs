// Answer 0

#[test]
fn test_matches_with_character_in_first_range() {
    let range = InstRanges {
        goto: 1,
        ranges: vec![('a', 'b'), ('c', 'd'), ('e', 'f'), ('g', 'h')],
    };
    let result = range.matches(Char(97)); // 'a'
    assert!(result);
}

#[test]
fn test_matches_with_character_exceeding_first_range() {
    let range = InstRanges {
        goto: 1,
        ranges: vec![('a', 'b'), ('c', 'd'), ('e', 'f'), ('g', 'h')],
    };
    let result = range.matches(Char(99)); // 'c'
    assert!(!result);
}

#[test]
fn test_matches_with_character_not_in_ranges() {
    let range = InstRanges {
        goto: 1,
        ranges: vec![('a', 'b'), ('c', 'd'), ('e', 'f'), ('g', 'h')],
    };
    let result = range.matches(Char(65)); // 'A'
    assert!(!result);
}

#[test]
fn test_matches_with_character_equal_to_second_range_lower_bound() {
    let range = InstRanges {
        goto: 1,
        ranges: vec![('a', 'b'), ('c', 'd'), ('e', 'f'), ('g', 'h')],
    };
    let result = range.matches(Char(99)); // 'c'
    assert!(result);
}

#[test]
fn test_matches_with_character_lower_than_first_range() {
    let range = InstRanges {
        goto: 1,
        ranges: vec![('a', 'b')],
    };
    let result = range.matches(Char(96)); // '<'
    assert!(!result);
}

#[test]
fn test_matches_with_character_at_upper_bound_of_first_range() {
    let range = InstRanges {
        goto: 1,
        ranges: vec![('a', 'b')],
    };
    let result = range.matches(Char(98)); // 'b'
    assert!(result);
}

#[test]
fn test_matches_with_character_exceeding_all_ranges() {
    let range = InstRanges {
        goto: 1,
        ranges: vec![('a', 'b'), ('c', 'd')],
    };
    let result = range.matches(Char(101)); // 'e'
    assert!(!result);
}


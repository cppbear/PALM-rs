// Answer 0

#[test]
fn test_matches_with_character_in_empty_ranges() {
    let inst_ranges = InstRanges {
        goto: 1,
        ranges: vec![],
    };
    let c = Char(65); // Character 'A'
    inst_ranges.matches(c);
}

#[test]
fn test_matches_with_character_outside_first_range() {
    let inst_ranges = InstRanges {
        goto: 1,
        ranges: vec![('b', 'd')],
    };
    let c = Char(97); // Character 'a'
    inst_ranges.matches(c);
}

#[test]
fn test_matches_with_character_in_first_range() {
    let inst_ranges = InstRanges {
        goto: 1,
        ranges: vec![('a', 'c')],
    };
    let c = Char(97); // Character 'a'
    inst_ranges.matches(c);
}

#[test]
fn test_matches_with_character_equal_to_first_range() {
    let inst_ranges = InstRanges {
        goto: 1,
        ranges: vec![('a', 'c')],
    };
    let c = Char(99); // Character 'c'
    inst_ranges.matches(c);
}

#[test]
fn test_matches_with_character_in_multiple_ranges() {
    let inst_ranges = InstRanges {
        goto: 1,
        ranges: vec![('a', 'c'), ('d', 'f'), ('g', 'i')],
    };
    let c = Char(101); // Character 'e'
    inst_ranges.matches(c);
}

#[test]
fn test_matches_with_character_after_last_range() {
    let inst_ranges = InstRanges {
        goto: 1,
        ranges: vec![('a', 'c'), ('d', 'f')],
    };
    let c = Char(103); // Character 'g'
    inst_ranges.matches(c);
}

#[test]
fn test_matches_with_character_in_panic_case() {
    let inst_ranges = InstRanges {
        goto: 1,
        ranges: vec![('a', 'c'), ('d', 'f')],
    };
    let c = Char(60); // Character '<' falls outside of defined ranges
    inst_ranges.matches(c);
}

#[test]
fn test_matches_with_character_in_only_in_range() {
    let inst_ranges = InstRanges {
        goto: 1,
        ranges: vec![('f', 'h')],
    };
    let c = Char(104); // Character 'h'
    inst_ranges.matches(c);
}


// Answer 0

#[test]
fn test_matches_less_than_first_range() {
    let inst_ranges = InstRanges {
        goto: 0,
        ranges: vec![('b', 'c'), ('d', 'f'), ('g', 'h'), ('i', 'j')],
    };
    let character = Char(97); // 'a' < 'b', should return false
    assert_eq!(inst_ranges.matches(character), false);
}

#[test]
fn test_matches_less_than_first_multiple_ranges() {
    let inst_ranges = InstRanges {
        goto: 1,
        ranges: vec![('e', 'f'), ('g', 'j'), ('k', 'l')],
    };
    let character = Char(100); // 'd' < 'e', should return false
    assert_eq!(inst_ranges.matches(character), false);
}

#[test]
fn test_matches_less_than_first_with_four_ranges() {
    let inst_ranges = InstRanges {
        goto: 2,
        ranges: vec![('x', 'y'), ('z', 'z'), ('a', 'c'), ('d', 'd')],
    };
    let character = Char(119); // 'w' < 'x', should return false
    assert_eq!(inst_ranges.matches(character), false);
}


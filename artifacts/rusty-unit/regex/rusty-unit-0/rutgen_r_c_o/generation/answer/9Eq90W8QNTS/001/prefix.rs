// Answer 0

#[test]
fn test_matches_less_than_first_range() {
    let char_to_test = Char(0); // c < r.0, where r.0 > c
    let instruction = InstRanges {
        goto: 1,
        ranges: vec![(Char(1), Char(2))],
    };
    instruction.matches(char_to_test);
}

#[test]
fn test_matches_less_than_second_range() {
    let char_to_test = Char(0); // c < r.0, where r.0 > c
    let instruction = InstRanges {
        goto: 2,
        ranges: vec![(Char(2), Char(3)), (Char(4), Char(5))],
    };
    instruction.matches(char_to_test);
}

#[test]
fn test_matches_less_than_third_range() {
    let char_to_test = Char(0); // c < r.0, where r.0 > c
    let instruction = InstRanges {
        goto: 3,
        ranges: vec![(Char(3), Char(4)), (Char(5), Char(6)), (Char(7), Char(8))],
    };
    instruction.matches(char_to_test);
}

#[test]
fn test_matches_less_than_fourth_range() {
    let char_to_test = Char(0); // c < r.0, where r.0 > c
    let instruction = InstRanges {
        goto: 4,
        ranges: vec![(Char(4), Char(5)), (Char(6), Char(7)), (Char(8), Char(9)), (Char(10), Char(11))],
    };
    instruction.matches(char_to_test);
}


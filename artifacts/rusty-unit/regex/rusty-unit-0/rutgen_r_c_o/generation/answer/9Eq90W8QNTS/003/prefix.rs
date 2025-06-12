// Answer 0

#[test]
fn test_matches_case_1() {
    let inst_ranges = InstRanges {
        goto: 1,
        ranges: vec![(char::from('\u{0000}'), char::from('\u{0001}'))],
    };
    let c = Char(0x0000);
    inst_ranges.matches(c);
}

#[test]
fn test_matches_case_2() {
    let inst_ranges = InstRanges {
        goto: 1,
        ranges: vec![(char::from('\u{0000}'), char::from('\u{0001}'))],
    };
    let c = Char(0x0001);
    inst_ranges.matches(c);
}

#[test]
fn test_matches_case_3() {
    let inst_ranges = InstRanges {
        goto: 1,
        ranges: vec![(char::from('\u{0000}'), char::from('\u{0001}'))],
    };
    let c = Char(0x0002);
    inst_ranges.matches(c);
}

#[test]
fn test_matches_case_4() {
    let inst_ranges = InstRanges {
        goto: 1,
        ranges: vec![(char::from('\u{0000}'), char::from('\u{0001}')),
                     (char::from('\u{0002}'), char::from('\u{0003}')),
                     (char::from('\u{0004}'), char::from('\u{0005}')),
                     (char::from('\u{0006}'), char::from('\u{0007}'))],
    };
    let c = Char(0x0003);
    inst_ranges.matches(c);
}

#[test]
fn test_matches_case_5() {
    let inst_ranges = InstRanges {
        goto: 1,
        ranges: vec![(char::from('\u{0000}'), char::from('\u{0001}')),
                     (char::from('\u{0002}'), char::from('\u{0003}')),
                     (char::from('\u{0004}'), char::from('\u{0005}')),
                     (char::from('\u{0006}'), char::from('\u{0007}'))],
    };
    let c = Char(0x0008);
    inst_ranges.matches(c);
}

#[test]
fn test_matches_empty_ranges() {
    let inst_ranges = InstRanges {
        goto: 1,
        ranges: vec![],
    };
    let c = Char(0x0000);
    inst_ranges.matches(c);
}


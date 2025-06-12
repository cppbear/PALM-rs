// Answer 0

#[test]
fn test_matches_with_lower_bound_char() {
    let ranges = vec![(char::from(0), char::from(1)), 
                      (char::from(1), char::from(2)), 
                      (char::from(2), char::from(3)), 
                      (char::from(3), char::from(4))];
    let inst_ranges = InstRanges { goto: 1, ranges };
    let c = Char(1);
    inst_ranges.matches(c);
}

#[test]
fn test_matches_with_mid_range_char() {
    let ranges = vec![(char::from(0), char::from(1)), 
                      (char::from(1), char::from(2)), 
                      (char::from(2), char::from(3)), 
                      (char::from(3), char::from(4))];
    let inst_ranges = InstRanges { goto: 1, ranges };
    let c = Char(2);
    inst_ranges.matches(c);
}

#[test]
fn test_matches_with_upper_bound_char() {
    let ranges = vec![(char::from(0), char::from(1)), 
                      (char::from(1), char::from(2)), 
                      (char::from(2), char::from(3)), 
                      (char::from(3), char::from(4))];
    let inst_ranges = InstRanges { goto: 1, ranges };
    let c = Char(3);
    inst_ranges.matches(c);
}

#[test]
fn test_matches_with_exact_upper_bound_char() {
    let ranges = vec![(char::from(0), char::from(1)), 
                      (char::from(1), char::from(2)), 
                      (char::from(2), char::from(3)), 
                      (char::from(3), char::from(4))];
    let inst_ranges = InstRanges { goto: 1, ranges };
    let c = Char(4);
    inst_ranges.matches(c);
}


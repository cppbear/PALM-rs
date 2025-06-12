// Answer 0

#[test]
fn test_set_non_match_zero() {
    let input = Result::NoMatch(0);
    let result = input.set_non_match(0);
}

#[test]
fn test_set_non_match_one() {
    let input = Result::NoMatch(1);
    let result = input.set_non_match(1);
}

#[test]
fn test_set_non_match_max() {
    let input = Result::NoMatch(usize::MAX);
    let result = input.set_non_match(usize::MAX);
}

#[test]
fn test_set_non_match_mid() {
    let input = Result::NoMatch(500);
    let result = input.set_non_match(500);
}

#[test]
fn test_set_non_match_double_update() {
    let input = Result::NoMatch(10);
    let first_result = input.set_non_match(20);
    let second_result = first_result.set_non_match(30);
}


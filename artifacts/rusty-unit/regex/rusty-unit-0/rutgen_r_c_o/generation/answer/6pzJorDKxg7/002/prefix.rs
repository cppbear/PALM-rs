// Answer 0

#[test]
fn test_no_match_zero() {
    let result = Result::NoMatch(0);
    result.is_match();
}

#[test]
fn test_no_match_one() {
    let result = Result::NoMatch(1);
    result.is_match();
}

#[test]
fn test_no_match_large() {
    let result = Result::NoMatch(4294967295);
    result.is_match();
}


// Answer 0

#[test]
fn test_is_match_with_zero() {
    let instruction = Inst::Match(0);
    instruction.is_match();
}

#[test]
fn test_is_match_with_one() {
    let instruction = Inst::Match(1);
    instruction.is_match();
}

#[test]
fn test_is_match_with_large_number() {
    let instruction = Inst::Match(1000);
    instruction.is_match();
}

#[test]
fn test_is_match_with_high_index() {
    let instruction = Inst::Match(99999);
    instruction.is_match();
}


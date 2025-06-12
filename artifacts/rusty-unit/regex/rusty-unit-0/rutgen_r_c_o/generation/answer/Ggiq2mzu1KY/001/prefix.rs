// Answer 0

#[test]
fn test_is_match_zero() {
    let state_flags = StateFlags(0);
    state_flags.is_match();
}

#[test]
fn test_is_match_one() {
    let state_flags = StateFlags(1);
    state_flags.is_match();
}

#[test]
fn test_is_match_two() {
    let state_flags = StateFlags(2);
    state_flags.is_match();
}

#[test]
fn test_is_match_four() {
    let state_flags = StateFlags(4);
    state_flags.is_match();
}

#[test]
fn test_is_match_eight() {
    let state_flags = StateFlags(8);
    state_flags.is_match();
}

#[test]
fn test_is_match_sixteen() {
    let state_flags = StateFlags(16);
    state_flags.is_match();
}

#[test]
fn test_is_match_thirty_two() {
    let state_flags = StateFlags(32);
    state_flags.is_match();
}

#[test]
fn test_is_match_sixty_four() {
    let state_flags = StateFlags(64);
    state_flags.is_match();
}

#[test]
fn test_is_match_one_hundred_twenty_eight() {
    let state_flags = StateFlags(128);
    state_flags.is_match();
}

#[test]
fn test_is_match_two_hundred_fifty_five() {
    let state_flags = StateFlags(255);
    state_flags.is_match();
}

#[test]
fn test_is_match_maximum_value() {
    let state_flags = StateFlags(255);
    state_flags.is_match();
}


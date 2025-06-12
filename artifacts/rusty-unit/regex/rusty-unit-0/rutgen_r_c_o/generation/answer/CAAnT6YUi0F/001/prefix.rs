// Answer 0

#[test]
fn test_set_match_initial_zero() {
    let mut state_flags = StateFlags(0);
    state_flags.set_match();
}

#[test]
fn test_set_match_initial_one() {
    let mut state_flags = StateFlags(1);
    state_flags.set_match();
}

#[test]
fn test_set_match_initial_max() {
    let mut state_flags = StateFlags(255);
    state_flags.set_match();
}

#[test]
fn test_set_match_initial_two_fifty_five() {
    let mut state_flags = StateFlags(255);
    state_flags.set_match();
    state_flags.set_match(); // sets it again, should still be valid
}

#[test]
fn test_set_match_initial_two() {
    let mut state_flags = StateFlags(2);
    state_flags.set_match();
}


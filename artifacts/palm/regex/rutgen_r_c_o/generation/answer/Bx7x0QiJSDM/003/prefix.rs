// Answer 0

#[test]
fn test_map_match_with_zero() {
    let result = Result::Match(0);
    let mapped_result = result.map(|x| x + 1);
}

#[test]
fn test_map_match_with_half_state_max() {
    let state = STATE_MAX / 2;
    let result = Result::Match(state);
    let mapped_result = result.map(|x| x * 2);
}

#[test]
fn test_map_match_with_state_max() {
    let result = Result::Match(STATE_MAX);
    let mapped_result = result.map(|x| x - 1);
}

#[test]
fn test_map_match_with_mid_point_state() {
    let state = STATE_MATCH / 2;
    let result = Result::Match(state);
    let mapped_result = result.map(|x| x + 10);
}

#[test]
fn test_map_match_with_one_less_than_state_max() {
    let result = Result::Match(STATE_MAX - 1);
    let mapped_result = result.map(|x| x + 5);
}


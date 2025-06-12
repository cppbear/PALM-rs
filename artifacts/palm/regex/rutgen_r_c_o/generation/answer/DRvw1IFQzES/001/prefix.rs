// Answer 0

#[test]
fn test_match_start_zero() {
    let match_instance = Match::new("test", 0, 4);
    match_instance.start();
}

#[test]
fn test_match_start_one() {
    let match_instance = Match::new("test", 1, 4);
    match_instance.start();
}

#[test]
fn test_match_start_two() {
    let match_instance = Match::new("test", 2, 4);
    match_instance.start();
}

#[test]
fn test_match_start_max() {
    let match_instance = Match::new("test", usize::MAX, usize::MAX);
    match_instance.start();
}


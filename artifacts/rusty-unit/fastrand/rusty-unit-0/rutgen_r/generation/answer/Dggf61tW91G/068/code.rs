// Answer 0

#[test]
#[should_panic(expected = "empty range")]
fn test_char_panic_empty_range_start_bound_excluded_lower_bound() {
    let mut rng = fastrand::Rng::new();
    rng.char(1..=1);
}

#[test]
#[should_panic(expected = "empty range")]
fn test_char_panic_empty_range_end_bound_excluded_upper_bound() {
    let mut rng = fastrand::Rng::new();
    rng.char(0..=0);
}

#[test]
fn test_char_same_low_high_with_excluded_bounds() {
    let mut rng = fastrand::Rng::new();
    let result = rng.char('a'..='a');
    assert_eq!(result, 'a');
}

#[test]
#[should_panic(expected = "empty range")]
fn test_char_panic_val_try_into() {
    let mut rng = fastrand::Rng::new();
    let result = rng.char('a'..='ÿ'); // an intentionally wide range to force checks
    assert!(result >= 'a' && result <= 'ÿ');
}

#[test]
fn test_char_gap_with_provided_bounds() {
    let mut rng = fastrand::Rng::new();
    let result = rng.char('\u{D7FF}'..='\u{D7FF}'); // boundaries leading to gap
    assert_eq!(result, '\u{D7FF}');
}


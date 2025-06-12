// Answer 0

#[test]
fn test_char_unbounded_range() {
    let mut rng = Rng::with_seed(1);
    let result = rng.char(..);
    assert!(result.is_ascii());
}

#[test]
fn test_char_low_equal_high() {
    let mut rng = Rng::with_seed(2);
    let result = rng.char('a'..='a');
    assert_eq!(result, 'a');
}

#[test]
fn test_char_surrogate_gap() {
    let mut rng = Rng::with_seed(3);
    let result = rng.char('\u{0100}'..='\u{D7FF}');
    assert!('\u{0100}' <= result && result <= '\u{D7FF}');
}

#[test]
#[should_panic(expected = "empty range")]
fn test_char_empty_excluded_low() {
    let mut rng = Rng::with_seed(4);
    let _result = rng.char('a'..='b');
    let _excluded = rng.char('b'..='b');
}

#[test]
#[should_panic(expected = "empty range")]
fn test_char_empty_excluded_high() {
    let mut rng = Rng::with_seed(5);
    let _result = rng.char('a'..='b');
    let _excluded = rng.char('a'..='a');
}


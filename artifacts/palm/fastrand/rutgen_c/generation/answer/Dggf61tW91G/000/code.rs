// Answer 0

#[test]
fn test_char_inclusive_range() {
    let mut rng = Rng::with_seed(42);
    let result = rng.char('a'..='z');
    assert!(result >= 'a' && result <= 'z');
}

#[test]
fn test_char_exclusive_range() {
    let mut rng = Rng::with_seed(42);
    let result = rng.char('a'..'z');
    assert!(result >= 'a' && result < 'z');
}

#[test]
fn test_char_unbounded_range() {
    let mut rng = Rng::with_seed(42);
    let result = rng.char(core::ops::Bound::Unbounded..='z');
    assert!(result <= 'z');
}

#[test]
fn test_char_empty_range_inclusive() {
    let mut rng = Rng::with_seed(42);
    let result = std::panic::catch_unwind(|| rng.char('a'..='a'));
    assert!(result.is_err());
}

#[test]
fn test_char_empty_range_exclusive() {
    let mut rng = Rng::with_seed(42);
    let result = std::panic::catch_unwind(|| rng.char('a'..'a'));
    assert!(result.is_err());
}

#[test]
fn test_char_range_with_surrogate() {
    let mut rng = Rng::with_seed(42);
    let result = rng.char('\u{D7FF}'..='\u{E000}');
    assert!(result >= '\u{D7FF}' && result <= '\u{E000}');
}


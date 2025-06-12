// Answer 0

#[test]
fn test_char_unbounded_range() {
    let mut rng = fastrand::Rng::new();
    let result = rng.char(..);
    assert!((result as u32) <= core::char::MAX as u32);
}

#[test]
fn test_char_inclusive_range() {
    let mut rng = fastrand::Rng::new();
    let result = rng.char('a'..='z');
    assert!(result >= 'a' && result <= 'z');
}

#[test]
fn test_char_exclusive_range() {
    let mut rng = fastrand::Rng::new();
    let result = rng.char('a'..'z');
    assert!(result >= 'b' && result < 'z');
}

#[test]
#[should_panic]
fn test_char_empty_inclusive_range() {
    let mut rng = fastrand::Rng::new();
    rng.char('a'..='a');
}

#[test]
#[should_panic]
fn test_char_empty_exclusive_range() {
    let mut rng = fastrand::Rng::new();
    rng.char('a'..'a');
}

#[test]
#[should_panic]
fn test_char_inverted_range() {
    let mut rng = fastrand::Rng::new();
    rng.char('z'..='a');
}


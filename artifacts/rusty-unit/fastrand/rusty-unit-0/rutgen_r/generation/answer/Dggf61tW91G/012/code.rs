// Answer 0

#[test]
fn test_char_with_unbounded_range() {
    let mut rng = fastrand::Rng::new();
    let result = rng.char(..);
    assert!(result.is_alphabetic() || result.is_numeric() || result.is_whitespace());
}

#[test]
fn test_char_with_equal_bounds() {
    let mut rng = fastrand::Rng::new();
    let result = rng.char('a'..='a');
    assert_eq!(result, 'a');
}

#[test]
fn test_char_with_surrealt_start() {
    let mut rng = fastrand::Rng::new();
    let result = rng.char(char::from_u32(0xd800).unwrap()..=char::from_u32(0xd800).unwrap());
    assert_eq!(result, char::from_u32(0xd800).unwrap());
}

#[test]
fn test_char_with_surrogate_gap() {
    let mut rng = fastrand::Rng::new();
    let result = rng.char('a'..='\\'); // This range includes the surrogate area
    assert!(result >= 'a' && result <= '\\');
}

#[test]
#[should_panic(expected = "empty range")]
fn test_char_empty_range_inclusion() {
    let mut rng = fastrand::Rng::new();
    rng.char('a'..='`'); // No characters in this range due to the excluded boundary
}

#[test]
#[should_panic(expected = "empty range")]
fn test_char_empty_range_exclusion() {
    let mut rng = fastrand::Rng::new();
    rng.char('a'..='a'); // Should panic because this is an empty range
}


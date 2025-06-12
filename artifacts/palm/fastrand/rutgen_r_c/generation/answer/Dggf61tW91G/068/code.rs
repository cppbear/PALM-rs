// Answer 0

#[test]
fn test_char_with_excluded_surrogate_start_minus_one() {
    let mut rng = Rng::with_seed(123);
    let result = rng.char(0u8 as char..=0u8 as char);
    assert_eq!(result, 0u8 as char);
}

#[test]
#[should_panic(expected = "empty range: Excluded(65533)..Excluded(65536)")]
fn test_char_with_excluded_surrogate_start_plus_length() {
    let mut rng = Rng::with_seed(123);
    let _ = rng.char(
        (0xd7ff as char)..=(0xd800 as char),
    );
}

#[test]
fn test_char_with_low_equals_high() {
    let mut rng = Rng::with_seed(123);
    let result = rng.char('a'..='a');
    assert_eq!(result, 'a');
}

#[test]
fn test_char_low_below_surrogate_high_at_surrogate() {
    let mut rng = Rng::with_seed(123);
    let result = rng.char('a'..='𐀀'); // 𐀀 is U+10000 which is equivalent to 0x10000
    assert!(result >= 'a' && result <= '𐀀');
}

#[test]
#[should_panic(expected = "empty range: Excluded(65535)..Excluded(65536)")]
fn test_char_with_low_higher_than_high() {
    let mut rng = Rng::with_seed(123);
    let _ = rng.char(0xc000 as char..=0xbfff as char);
}


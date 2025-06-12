// Answer 0

#[test]
fn test_char_unbounded_range() {
    let mut rng = Rng::with_seed(1234);
    rng.char(..);
}

#[test]
fn test_char_equal_bounds() {
    let mut rng = Rng::with_seed(1234);
    rng.char('a'..='a');
}

#[test]
fn test_char_surrogate_start_equal_bounds() {
    let mut rng = Rng::with_seed(1234);
    rng.char(char::try_from(0xd800).unwrap()..=char::try_from(0xd800).unwrap());
}

#[test]
fn test_char_surrogate_start_included() {
    let mut rng = Rng::with_seed(1234);
    rng.char(char::try_from(0xd800).unwrap()..=char::try_from(0xd801).unwrap());
}

#[test]
fn test_char_surrogate_start_excluded() {
    let mut rng = Rng::with_seed(1234);
    rng.char(char::try_from(0xd799).unwrap()..=char::try_from(0xd800).unwrap());
}

#[test]
#[should_panic(expected = "empty range")]
fn test_char_empty_range() {
    let mut rng = Rng::with_seed(1234);
    rng.char('z'..='a');
}


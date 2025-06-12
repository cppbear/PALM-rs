// Answer 0

#[test]
fn test_char_excluded_bounds_success_case() {
    let mut rng = Rng::with_seed(42);
    rng.char(
        (char::try_from(0xd7ff).unwrap(), char::try_from(0xdc00).unwrap())
    );
}

#[test]
#[should_panic(expected = "empty range: Bound::Excluded(0xd7ff)..Bound::Excluded(0xd800)")]
fn test_char_excluded_bounds_panic_case() {
    let mut rng = Rng::with_seed(42);
    rng.char(
        (char::try_from(0xd800).unwrap(), char::try_from(0xd800).unwrap())
    );
}

#[test]
fn test_char_inclusive_bounds_same_value() {
    let mut rng = Rng::with_seed(42);
    rng.char(
        (char::try_from(0xdc00).unwrap(), char::try_from(0xdc00).unwrap())
    );
}

#[test]
fn test_char_surrogate_gap_no_inclusivity() {
    let mut rng = Rng::with_seed(42);
    rng.char(
        (char::try_from(0xd780).unwrap(), char::try_from(0xd7ff).unwrap())
    );
}

#[test]
fn test_char_range_beyond_surrogate_gap() {
    let mut rng = Rng::with_seed(42);
    rng.char(
        (char::try_from(0xd7fe).unwrap(), char::try_from(0xdc01).unwrap())
    );
}


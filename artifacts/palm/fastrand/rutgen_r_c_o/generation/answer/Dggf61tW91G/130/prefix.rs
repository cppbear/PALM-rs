// Answer 0

#[test]
fn test_char_range_inclusive() {
    let mut rng = Rng::with_seed(1);
    let _ = rng.char(0x0020..=0xD7FF);
}

#[test]
fn test_char_range_exclusive() {
    let mut rng = Rng::with_seed(2);
    let _ = rng.char(0x0021..0xD7FF);
}

#[test]
#[should_panic]
fn test_char_empty_range_low_higher_than_high() {
    let mut rng = Rng::with_seed(3);
    let _ = rng.char(0xD7FF..0x0020);
}

#[test]
#[should_panic]
fn test_char_range_inclusive_surrogate_start_excluded() {
    let mut rng = Rng::with_seed(4);
    let _ = rng.char(0x0020..=0xD800);
}

#[test]
fn test_char_range_exclusive_surrogate_end() {
    let mut rng = Rng::with_seed(5);
    let _ = rng.char(0x0020..0xD800);
}


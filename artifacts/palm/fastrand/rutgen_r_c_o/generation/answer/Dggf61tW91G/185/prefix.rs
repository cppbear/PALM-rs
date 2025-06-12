// Answer 0

#[test]
fn test_char_included_bounds_regular_range() {
    let mut rng = Rng::with_seed(42);
    rng.char('a'..='z');
}

#[test]
fn test_char_included_bounds_surrogate_edge_case() {
    let mut rng = Rng::with_seed(42);
    rng.char('𐀀'..='𐁿');
}

#[test]
fn test_char_included_bounds_across_surrogate() {
    let mut rng = Rng::with_seed(42);
    rng.char('𐁀'..='𐇀');
}

#[test]
fn test_char_included_bounds_in_surrogate() {
    let mut rng = Rng::with_seed(42);
    rng.char('𐇁'..='𐍀');
}

#[test]
#[should_panic]
fn test_char_empty_range_low_greater_than_high() {
    let mut rng = Rng::with_seed(42);
    rng.char('z'..='a');
}

#[test]
fn test_char_exact_surrogate_start() {
    let mut rng = Rng::with_seed(42);
    rng.char('𐌀'..='𐓝');
}

#[test]
fn test_char_with_high_equals_surrogate_start() {
    let mut rng = Rng::with_seed(42);
    rng.char('𐍀'..='𐍿');
}


// Answer 0

#[test]
fn test_new_with_default_values() {
    struct Lcg128Xsl64 {
        state: u128,
        increment: u128,
    }

    impl Lcg128Xsl64 {
        fn from_state_incr(state: u128, increment: u128) -> Self {
            Lcg128Xsl64 { state, increment }
        }
    }

    let state = 0xcafef00dd15ea5e5;
    let stream = 0xa02bdbf7bb3c0a7ac28fa16a64abf96;
    let generator = new(state, stream);

    assert_eq!(generator.state, state);
    assert_eq!(generator.increment, (stream << 1) | 1);
}

#[test]
fn test_new_with_zero_stream() {
    struct Lcg128Xsl64 {
        state: u128,
        increment: u128,
    }

    impl Lcg128Xsl64 {
        fn from_state_incr(state: u128, increment: u128) -> Self {
            Lcg128Xsl64 { state, increment }
        }
    }

    let state = 0xcafef00dd15ea5e5;
    let stream = 0;
    let generator = new(state, stream);

    assert_eq!(generator.state, state);
    assert_eq!(generator.increment, (stream << 1) | 1);
}

#[test]
fn test_new_with_max_stream() {
    struct Lcg128Xsl64 {
        state: u128,
        increment: u128,
    }

    impl Lcg128Xsl64 {
        fn from_state_incr(state: u128, increment: u128) -> Self {
            Lcg128Xsl64 { state, increment }
        }
    }

    let state = 0xcafef00dd15ea5e5;
    let stream = u128::MAX;
    let generator = new(state, stream);

    assert_eq!(generator.state, state);
    assert_eq!(generator.increment, (stream << 1) | 1);
}


// Answer 0

#[test]
fn test_new_default_values() {
    struct PcgInstance {
        state: u128,
        stream: u128,
    }

    impl PcgInstance {
        fn from_state_incr(state: u128, increment: u128) -> Self {
            PcgInstance { state, stream: increment }
        }

        fn new(state: u128, stream: u128) -> Self {
            let increment = (stream << 1) | 1;
            Self::from_state_incr(state, increment)
        }
    }

    let instance = PcgInstance::new(0xcafef00dd15ea5e5, 0xa02bdbf7bb3c0a7ac28fa16a64abf96);
    assert_eq!(instance.state, 0xcafef00dd15ea5e5);
    assert_eq!(instance.stream, 0xa02bdbf7bb3c0a7ac28fa16a64abf96 << 1 | 1);
}

#[test]
fn test_new_edge_case() {
    struct PcgInstance {
        state: u128,
        stream: u128,
    }

    impl PcgInstance {
        fn from_state_incr(state: u128, increment: u128) -> Self {
            PcgInstance { state, stream: increment }
        }

        fn new(state: u128, stream: u128) -> Self {
            let increment = (stream << 1) | 1;
            Self::from_state_incr(state, increment)
        }
    }

    let instance = PcgInstance::new(0, 0);
    assert_eq!(instance.state, 0);
    assert_eq!(instance.stream, 0 << 1 | 1);
}


// Answer 0

#[test]
fn test_state_heap_size() {
    use std::mem;

    struct StatePtr;

    struct State {
        num_byte_classes: usize,
    }

    impl State {
        fn state_heap_size(&self) -> usize {
            self.num_byte_classes * mem::size_of::<StatePtr>()
        }
    }

    // Test when num_byte_classes is 0
    let state_zero = State { num_byte_classes: 0 };
    assert_eq!(state_zero.state_heap_size(), 0);

    // Test when num_byte_classes is 1
    let state_one = State { num_byte_classes: 1 };
    assert_eq!(state_one.state_heap_size(), mem::size_of::<StatePtr>());

    // Test when num_byte_classes is 10
    let state_ten = State { num_byte_classes: 10 };
    assert_eq!(state_ten.state_heap_size(), 10 * mem::size_of::<StatePtr>());
}


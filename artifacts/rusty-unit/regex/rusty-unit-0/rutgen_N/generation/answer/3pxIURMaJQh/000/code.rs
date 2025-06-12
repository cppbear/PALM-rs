// Answer 0

#[test]
fn test_next_unchecked_within_bounds() {
    struct DummyDFA {
        table: Vec<StatePtr>,
        num_byte_classes: usize,
    }

    impl DummyDFA {
        unsafe fn next_unchecked(&self, si: StatePtr, cls: usize) -> StatePtr {
            debug_assert!((si as usize) < self.table.len());
            debug_assert!(cls < self.num_byte_classes);
            *self.table.get_unchecked(si as usize + cls)
        }
    }

    let dfa = DummyDFA {
        table: vec![0, 1, 2, 3, 4],
        num_byte_classes: 5,
    };

    unsafe {
        let result = dfa.next_unchecked(1, 2);
        assert_eq!(result, 3);
    }
}

#[test]
#[should_panic]
fn test_next_unchecked_out_of_bounds_si() {
    struct DummyDFA {
        table: Vec<StatePtr>,
        num_byte_classes: usize,
    }

    impl DummyDFA {
        unsafe fn next_unchecked(&self, si: StatePtr, cls: usize) -> StatePtr {
            debug_assert!((si as usize) < self.table.len());
            debug_assert!(cls < self.num_byte_classes);
            *self.table.get_unchecked(si as usize + cls)
        }
    }

    let dfa = DummyDFA {
        table: vec![0, 1, 2],
        num_byte_classes: 3,
    };

    unsafe {
        // This should panic due to `si` being out of bounds
        let _ = dfa.next_unchecked(3, 0);
    }
}

#[test]
#[should_panic]
fn test_next_unchecked_out_of_bounds_cls() {
    struct DummyDFA {
        table: Vec<StatePtr>,
        num_byte_classes: usize,
    }

    impl DummyDFA {
        unsafe fn next_unchecked(&self, si: StatePtr, cls: usize) -> StatePtr {
            debug_assert!((si as usize) < self.table.len());
            debug_assert!(cls < self.num_byte_classes);
            *self.table.get_unchecked(si as usize + cls)
        }
    }

    let dfa = DummyDFA {
        table: vec![0, 1, 2],
        num_byte_classes: 2,
    };

    unsafe {
        // This should panic due to `cls` being out of bounds
        let _ = dfa.next_unchecked(1, 2);
    }
}


// Answer 0

#[test]
fn test_is_empty_true() {
    struct SetSlice {
        entries: Vec<i32>,
    }

    impl SetSlice {
        pub const fn is_empty(&self) -> bool {
            self.entries.is_empty()
        }
    }

    let set_slice = SetSlice { entries: Vec::new() };
    assert!(set_slice.is_empty());
}

#[test]
fn test_is_empty_false() {
    struct SetSlice {
        entries: Vec<i32>,
    }

    impl SetSlice {
        pub const fn is_empty(&self) -> bool {
            self.entries.is_empty()
        }
    }

    let set_slice = SetSlice { entries: vec![1, 2, 3] };
    assert!(!set_slice.is_empty());
}


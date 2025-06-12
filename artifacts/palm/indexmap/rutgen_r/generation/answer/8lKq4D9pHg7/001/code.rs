// Answer 0

#[test]
fn test_split_at_mut_valid_index() {
    struct TestSlice {
        entries: Vec<i32>,
    }

    impl TestSlice {
        fn from_mut_slice(slice: &mut [i32]) -> &mut Self {
            unsafe { &mut *(slice as *mut [i32] as *mut Self) }
        }

        fn split_at_mut(&mut self, index: usize) -> (&mut Self, &mut Self) {
            let (first, second) = self.entries.split_at_mut(index);
            (Self::from_mut_slice(first), Self::from_mut_slice(second))
        }
    }

    let mut test_slice = TestSlice {
        entries: vec![1, 2, 3, 4, 5],
    };

    let (left, right) = test_slice.split_at_mut(3);
    assert_eq!(left.entries, vec![1, 2, 3]);
    assert_eq!(right.entries, vec![4, 5]);
}

#[test]
#[should_panic]
fn test_split_at_mut_panic_index_greater_than_length() {
    struct TestSlice {
        entries: Vec<i32>,
    }

    impl TestSlice {
        fn from_mut_slice(slice: &mut [i32]) -> &mut Self {
            unsafe { &mut *(slice as *mut [i32] as *mut Self) }
        }

        fn split_at_mut(&mut self, index: usize) -> (&mut Self, &mut Self) {
            let (first, second) = self.entries.split_at_mut(index);
            (Self::from_mut_slice(first), Self::from_mut_slice(second))
        }
    }

    let mut test_slice = TestSlice {
        entries: vec![1, 2, 3],
    };

    // This should panic as the index is greater than the length of entries.
    let _ = test_slice.split_at_mut(4);
}

#[test]
fn test_split_at_mut_index_equal_to_length() {
    struct TestSlice {
        entries: Vec<i32>,
    }

    impl TestSlice {
        fn from_mut_slice(slice: &mut [i32]) -> &mut Self {
            unsafe { &mut *(slice as *mut [i32] as *mut Self) }
        }

        fn split_at_mut(&mut self, index: usize) -> (&mut Self, &mut Self) {
            let (first, second) = self.entries.split_at_mut(index);
            (Self::from_mut_slice(first), Self::from_mut_slice(second))
        }
    }

    let mut test_slice = TestSlice {
        entries: vec![1, 2, 3],
    };

    let (left, right) = test_slice.split_at_mut(3);
    assert_eq!(left.entries, vec![1, 2, 3]);
    assert_eq!(right.entries, Vec::<i32>::new());
}


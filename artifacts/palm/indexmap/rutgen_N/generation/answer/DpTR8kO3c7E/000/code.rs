// Answer 0

#[test]
fn test_split_at_valid_index() {
    struct TestSlice {
        entries: Vec<i32>,
    }
    
    impl TestSlice {
        fn from_slice(slice: &[i32]) -> &Self {
            unsafe { &*(slice as *const [i32] as *const Self) }
        }
    }
    
    let slice = TestSlice {
        entries: vec![1, 2, 3, 4, 5],
    };
    let (first, second) = slice.split_at(3);
    assert_eq!(first.entries, vec![1, 2, 3]);
    assert_eq!(second.entries, vec![4, 5]);
}

#[test]
#[should_panic(expected = "index > len")]
fn test_split_at_index_too_large() {
    struct TestSlice {
        entries: Vec<i32>,
    }

    impl TestSlice {
        fn from_slice(slice: &[i32]) -> &Self {
            unsafe { &*(slice as *const [i32] as *const Self) }
        }
    }

    let slice = TestSlice {
        entries: vec![1, 2, 3],
    };
    let _ = slice.split_at(4);
}

#[test]
fn test_split_at_zero_index() {
    struct TestSlice {
        entries: Vec<i32>,
    }

    impl TestSlice {
        fn from_slice(slice: &[i32]) -> &Self {
            unsafe { &*(slice as *const [i32] as *const Self) }
        }
    }

    let slice = TestSlice {
        entries: vec![1, 2, 3, 4, 5],
    };
    let (first, second) = slice.split_at(0);
    assert_eq!(first.entries, vec![]);
    assert_eq!(second.entries, vec![1, 2, 3, 4, 5]);
}


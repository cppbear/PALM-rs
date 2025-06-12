// Answer 0

#[test]
fn test_iter_empty_set_slice() {
    struct SetSlice<T> {
        entries: Vec<T>,
    }

    impl<T> SetSlice<T> {
        pub fn iter(&self) -> std::slice::Iter<'_, T> {
            self.entries.iter()
        }
    }

    let set_slice: SetSlice<i32> = SetSlice { entries: Vec::new() };
    let mut iter = set_slice.iter();
    assert!(iter.next().is_none());
}

#[test]
fn test_iter_non_empty_set_slice() {
    struct SetSlice<T> {
        entries: Vec<T>,
    }

    impl<T> SetSlice<T> {
        pub fn iter(&self) -> std::slice::Iter<'_, T> {
            self.entries.iter()
        }
    }

    let set_slice = SetSlice { entries: vec![1, 2, 3] };
    let mut iter = set_slice.iter();
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert!(iter.next().is_none());
}


// Answer 0

#[test]
fn test_shift_remove_existing_value() {
    struct TestSet {
        data: Vec<i32>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet { data: vec![1, 2, 3, 4, 5] }
        }

        fn shift_remove<Q>(&mut self, value: &Q) -> bool
        where
            Q: ?Sized + PartialEq<i32>,
        {
            if let Some(pos) = self.data.iter().position(|x| x == value) {
                self.data.remove(pos);
                true
            } else {
                false
            }
        }
    }

    let mut set = TestSet::new();
    assert_eq!(set.shift_remove(&3), true);
    assert_eq!(set.data, vec![1, 2, 4, 5]);

    assert_eq!(set.shift_remove(&1), true);
    assert_eq!(set.data, vec![2, 4, 5]);

    assert_eq!(set.shift_remove(&5), true);
    assert_eq!(set.data, vec![2, 4]);

    assert_eq!(set.shift_remove(&7), false);
    assert_eq!(set.data, vec![2, 4]);
}

#[test]
fn test_shift_remove_non_existing_value() {
    struct TestSet {
        data: Vec<i32>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet { data: vec![1, 2, 3, 4, 5] }
        }

        fn shift_remove<Q>(&mut self, value: &Q) -> bool
        where
            Q: ?Sized + PartialEq<i32>,
        {
            if let Some(pos) = self.data.iter().position(|x| x == value) {
                self.data.remove(pos);
                true
            } else {
                false
            }
        }
    }

    let mut set = TestSet::new();
    assert_eq!(set.shift_remove(&6), false);
    assert_eq!(set.data, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_shift_remove_empty_set() {
    struct TestSet {
        data: Vec<i32>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet { data: Vec::new() }
        }

        fn shift_remove<Q>(&mut self, value: &Q) -> bool
        where
            Q: ?Sized + PartialEq<i32>,
        {
            if let Some(pos) = self.data.iter().position(|x| x == value) {
                self.data.remove(pos);
                true
            } else {
                false
            }
        }
    }

    let mut set = TestSet::new();
    assert_eq!(set.shift_remove(&1), false);
}


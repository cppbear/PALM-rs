// Answer 0

#[test]
fn test_pop_empty_index_set() {
    struct TestIndexSet {
        data: Vec<()>,
    }

    impl TestIndexSet {
        fn pop(&mut self) -> Option<()> {
            if self.data.is_empty() {
                None
            } else {
                Some(self.data.pop().unwrap())
            }
        }
    }

    let mut index_set = TestIndexSet { data: Vec::new() };
    assert_eq!(index_set.pop(), None);
}

#[test]
fn test_pop_single_element() {
    struct TestIndexSet {
        data: Vec<()>,
    }

    impl TestIndexSet {
        fn pop(&mut self) -> Option<()> {
            if self.data.is_empty() {
                None
            } else {
                Some(self.data.pop().unwrap())
            }
        }
    }

    let mut index_set = TestIndexSet { data: vec![()] };
    assert_eq!(index_set.pop(), Some(()));
    assert_eq!(index_set.pop(), None);
}

#[test]
fn test_pop_multiple_elements() {
    struct TestIndexSet {
        data: Vec<()>,
    }

    impl TestIndexSet {
        fn pop(&mut self) -> Option<()> {
            if self.data.is_empty() {
                None
            } else {
                Some(self.data.pop().unwrap())
            }
        }
    }

    let mut index_set = TestIndexSet { data: vec![(), (), ()] };
    assert_eq!(index_set.pop(), Some(()));
    assert_eq!(index_set.pop(), Some(()));
    assert_eq!(index_set.pop(), Some(()));
    assert_eq!(index_set.pop(), None);
}


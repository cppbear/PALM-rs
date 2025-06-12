// Answer 0

#[test]
fn test_take_exists() {
    struct TestSet {
        data: Vec<i32>,
    }

    impl TestSet {
        fn new(values: Vec<i32>) -> Self {
            TestSet { data: values }
        }

        fn take(&mut self, value: &i32) -> Option<i32> {
            if let Some(pos) = self.data.iter().position(|x| x == value) {
                Some(self.data.swap_remove(pos))
            } else {
                None
            }
        }
    }

    let mut set = TestSet::new(vec![1, 2, 3, 4]);
    let result = set.take(&2);
    assert_eq!(result, Some(2));
    assert_eq!(set.data, vec![1, 4, 3]);

    let result_none = set.take(&5);
    assert_eq!(result_none, None);
    assert_eq!(set.data, vec![1, 4, 3]);
}

#[test]
fn test_take_empty_set() {
    struct TestSet {
        data: Vec<i32>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet { data: Vec::new() }
        }

        fn take(&mut self, value: &i32) -> Option<i32> {
            if let Some(pos) = self.data.iter().position(|x| x == value) {
                Some(self.data.swap_remove(pos))
            } else {
                None
            }
        }
    }

    let mut set = TestSet::new();
    let result = set.take(&1);
    assert_eq!(result, None);
}


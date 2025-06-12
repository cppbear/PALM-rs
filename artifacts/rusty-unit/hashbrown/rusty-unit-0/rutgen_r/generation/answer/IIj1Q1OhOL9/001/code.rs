// Answer 0

#[test]
fn test_iter_clone_valid() {
    struct TestRaw {
        iter: Vec<i32>,
    }

    impl TestRaw {
        pub fn new(iter: Vec<i32>) -> Self {
            TestRaw { iter }
        }

        pub fn iter(&self) -> Vec<i32> {
            self.iter.clone()
        }
    }

    let raw = TestRaw::new(vec![1, 2, 3, 4, 5]);
    let cloned_iter = raw.iter();
    assert_eq!(cloned_iter, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_iter_empty() {
    struct TestRaw {
        iter: Vec<i32>,
    }

    impl TestRaw {
        pub fn new(iter: Vec<i32>) -> Self {
            TestRaw { iter }
        }

        pub fn iter(&self) -> Vec<i32> {
            self.iter.clone()
        }
    }

    let raw = TestRaw::new(vec![]);
    let cloned_iter = raw.iter();
    assert_eq!(cloned_iter, vec![]);
}

#[test]
fn test_iter_single_element() {
    struct TestRaw {
        iter: Vec<i32>,
    }

    impl TestRaw {
        pub fn new(iter: Vec<i32>) -> Self {
            TestRaw { iter }
        }

        pub fn iter(&self) -> Vec<i32> {
            self.iter.clone()
        }
    }

    let raw = TestRaw::new(vec![42]);
    let cloned_iter = raw.iter();
    assert_eq!(cloned_iter, vec![42]);
}


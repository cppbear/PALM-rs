// Answer 0

#[test]
fn test_sort_unstable_empty_set() {
    struct TestSet {
        map: Vec<i32>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet { map: vec![] }
        }

        fn sort_unstable(&mut self) {
            self.map.sort_unstable();
        }

        fn as_slice(&self) -> &[i32] {
            &self.map
        }
    }

    let mut set = TestSet::new();
    set.sort_unstable();
    assert!(set.as_slice().is_empty());
}

#[test]
fn test_sort_unstable_single_element() {
    struct TestSet {
        map: Vec<i32>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet { map: vec![] }
        }

        fn sort_unstable(&mut self) {
            self.map.sort_unstable();
        }

        fn push(&mut self, value: i32) {
            self.map.push(value);
        }

        fn as_slice(&self) -> &[i32] {
            &self.map
        }
    }

    let mut set = TestSet::new();
    set.push(5);
    set.sort_unstable();
    assert_eq!(set.as_slice(), &[5]);
}

#[test]
fn test_sort_unstable_multiple_elements() {
    struct TestSet {
        map: Vec<i32>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet { map: vec![] }
        }

        fn sort_unstable(&mut self) {
            self.map.sort_unstable();
        }

        fn push(&mut self, value: i32) {
            self.map.push(value);
        }

        fn as_slice(&self) -> &[i32] {
            &self.map
        }
    }

    let mut set = TestSet::new();
    set.push(3);
    set.push(1);
    set.push(2);
    set.sort_unstable();
    assert_eq!(set.as_slice(), &[1, 2, 3]);
}

#[test]
fn test_sort_unstable_with_duplicates() {
    struct TestSet {
        map: Vec<i32>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet { map: vec![] }
        }

        fn sort_unstable(&mut self) {
            self.map.sort_unstable();
        }

        fn push(&mut self, value: i32) {
            self.map.push(value);
        }

        fn as_slice(&self) -> &[i32] {
            &self.map
        }
    }

    let mut set = TestSet::new();
    set.push(2);
    set.push(3);
    set.push(2);
    set.sort_unstable();
    assert_eq!(set.as_slice(), &[2, 2, 3]);
}

#[test]
fn test_sort_unstable_negative_numbers() {
    struct TestSet {
        map: Vec<i32>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet { map: vec![] }
        }

        fn sort_unstable(&mut self) {
            self.map.sort_unstable();
        }

        fn push(&mut self, value: i32) {
            self.map.push(value);
        }

        fn as_slice(&self) -> &[i32] {
            &self.map
        }
    }

    let mut set = TestSet::new();
    set.push(-1);
    set.push(-3);
    set.push(-2);
    set.sort_unstable();
    assert_eq!(set.as_slice(), &[-3, -2, -1]);
}

#[test]
fn test_sort_unstable_floats() {
    struct TestSet {
        map: Vec<f32>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet { map: vec![] }
        }

        fn sort_unstable(&mut self) {
            self.map.sort_unstable();
        }

        fn push(&mut self, value: f32) {
            self.map.push(value);
        }

        fn as_slice(&self) -> &[f32] {
            &self.map
        }
    }

    let mut set = TestSet::new();
    set.push(3.1);
    set.push(2.2);
    set.push(1.3);
    set.sort_unstable();
    assert_eq!(set.as_slice(), &[1.3, 2.2, 3.1]);
}


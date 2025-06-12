// Answer 0

#[test]
fn test_swap_indices_valid() {
    struct TestSet {
        data: Vec<usize>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet { data: vec![1, 2, 3, 4] }
        }

        fn swap_indices(&mut self, a: usize, b: usize) {
            let len = self.data.len();
            assert!(a < len);
            assert!(b < len);
            self.data.swap(a, b);
        }

        fn as_slice(&self) -> &[usize] {
            &self.data
        }
    }

    let mut set = TestSet::new();
    set.swap_indices(0, 2);
    assert_eq!(set.as_slice(), &[3, 2, 1, 4]);
}

#[test]
#[should_panic]
fn test_swap_indices_index_out_of_bounds_a() {
    struct TestSet {
        data: Vec<usize>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet { data: vec![1, 2, 3] }
        }

        fn swap_indices(&mut self, a: usize, b: usize) {
            let len = self.data.len();
            assert!(a < len);
            assert!(b < len);
            self.data.swap(a, b);
        }
    }

    let mut set = TestSet::new();
    set.swap_indices(3, 1);
}

#[test]
#[should_panic]
fn test_swap_indices_index_out_of_bounds_b() {
    struct TestSet {
        data: Vec<usize>,
    }

    impl TestSet {
        fn new() -> Self {
            TestSet { data: vec![1, 2, 3] }
        }

        fn swap_indices(&mut self, a: usize, b: usize) {
            let len = self.data.len();
            assert!(a < len);
            assert!(b < len);
            self.data.swap(a, b);
        }
    }

    let mut set = TestSet::new();
    set.swap_indices(0, 3);
}


// Answer 0

#[test]
fn test_as_slice_non_empty() {
    struct TestIter {
        data: Vec<i32>,
        index: usize,
    }
    
    impl TestIter {
        fn new(data: Vec<i32>) -> Self {
            TestIter { data, index: 0 }
        }

        fn as_slice(&self) -> &[i32] {
            &self.data[self.index..]
        }
    }
    
    struct TestStruct<'a> {
        iter: &'a TestIter,
    }
    
    impl<'a> TestStruct<'a> {
        fn as_slice(&self) -> &'a [i32] {
            self.iter.as_slice()
        }
    }
    
    let test_iter = TestIter::new(vec![1, 2, 3, 4]);
    let test_struct = TestStruct { iter: &test_iter };
    let result = test_struct.as_slice();
    
    assert_eq!(result, &[1, 2, 3, 4]);
}

#[test]
fn test_as_slice_empty() {
    struct TestIter {
        data: Vec<i32>,
        index: usize,
    }

    impl TestIter {
        fn new(data: Vec<i32>) -> Self {
            TestIter { data, index: 0 }
        }

        fn as_slice(&self) -> &[i32] {
            &self.data[self.index..]
        }
    }

    struct TestStruct<'a> {
        iter: &'a TestIter,
    }

    impl<'a> TestStruct<'a> {
        fn as_slice(&self) -> &'a [i32] {
            self.iter.as_slice()
        }
    }

    let test_iter = TestIter::new(vec![]);
    let test_struct = TestStruct { iter: &test_iter };
    let result = test_struct.as_slice();
    
    assert_eq!(result, &[]);
}

#[should_panic]
#[test]
fn test_as_slice_panics_out_of_bounds() {
    struct TestIter {
        data: Vec<i32>,
        index: usize,
    }

    impl TestIter {
        fn new(data: Vec<i32>) -> Self {
            TestIter { data, index: 1 }
        }

        fn as_slice(&self) -> &[i32] {
            &self.data[self.index..]
        }
    }

    struct TestStruct<'a> {
        iter: &'a TestIter,
    }

    impl<'a> TestStruct<'a> {
        fn as_slice(&self) -> &'a [i32] {
            self.iter.as_slice()
        }
    }

    let test_iter = TestIter::new(vec![0]);
    let test_struct = TestStruct { iter: &test_iter };
    let _result = test_struct.as_slice(); // This should panic due to out of bounds
}


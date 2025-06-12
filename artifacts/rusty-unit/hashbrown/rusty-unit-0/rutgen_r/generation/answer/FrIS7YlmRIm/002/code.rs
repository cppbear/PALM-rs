// Answer 0

#[test]
fn test_next_impl_do_check_ptr_range_false() {
    struct TestBucket;

    struct TestData {
        index: usize,
        data: Vec<TestBucket>,
    }

    impl TestData {
        fn next_n(&mut self, index: usize) -> TestBucket {
            self.data[index].clone()
        }
    }

    struct TestGroup {
        indices: Vec<usize>,
        current_index: usize,
    }

    impl TestGroup {
        fn next(&mut self) -> Option<usize> {
            if self.current_index < self.indices.len() {
                let index = self.indices[self.current_index];
                self.current_index += 1;
                Some(index)
            } else {
                None
            }
        }
    }

    struct TestIterator {
        current_group: TestGroup,
        data: TestData,
        next_ctrl: usize,
        end: usize,
    }

    impl TestIterator {
        unsafe fn next_impl<const DO_CHECK_PTR_RANGE: bool>(&mut self) -> Option<TestBucket> {
            loop {
                if let Some(index) = self.current_group.next() {
                    return Some(self.data.next_n(index));
                }

                if DO_CHECK_PTR_RANGE && self.next_ctrl >= self.end {
                    return None;
                }

                self.current_group = TestGroup { 
                    indices: vec![0, 1, 2], // Simulate a group with indices
                    current_index: 0 
                };
                self.data = TestData { 
                    index: 0,
                    data: vec![TestBucket, TestBucket, TestBucket] 
                };
                self.next_ctrl += 3; // Simulate GROUP WIDTH
            }
        }
    }

    let mut iterator = TestIterator {
        current_group: TestGroup { indices: vec![], current_index: 0 },
        data: TestData { index: 0, data: vec![] },
        next_ctrl: 0,
        end: 9,
    };

    let result = unsafe { iterator.next_impl::<true>() };
    assert!(result.is_some());
}

#[test]
fn test_next_impl_do_check_ptr_range_true() {
    struct TestBucket;

    struct TestData {
        index: usize,
        data: Vec<TestBucket>,
    }

    impl TestData {
        fn next_n(&mut self, index: usize) -> TestBucket {
            self.data[index].clone()
        }
    }

    struct TestGroup {
        indices: Vec<usize>,
        current_index: usize,
    }

    impl TestGroup {
        fn next(&mut self) -> Option<usize> {
            if self.current_index < self.indices.len() {
                let index = self.indices[self.current_index];
                self.current_index += 1;
                Some(index)
            } else {
                None
            }
        }
    }

    struct TestIterator {
        current_group: TestGroup,
        data: TestData,
        next_ctrl: usize,
        end: usize,
    }

    impl TestIterator {
        unsafe fn next_impl<const DO_CHECK_PTR_RANGE: bool>(&mut self) -> Option<TestBucket> {
            loop {
                if let Some(index) = self.current_group.next() {
                    return Some(self.data.next_n(index));
                }

                if DO_CHECK_PTR_RANGE && self.next_ctrl >= self.end {
                    return None;
                }

                self.current_group = TestGroup { 
                    indices: vec![0, 1, 2], // Simulate a group with indices
                    current_index: 0 
                };
                self.data = TestData { 
                    index: 0,
                    data: vec![TestBucket, TestBucket, TestBucket] 
                };
                self.next_ctrl += 3; // Simulate GROUP WIDTH
            }
        }
    }

    let mut iterator = TestIterator {
        current_group: TestGroup { indices: vec![0, 1, 2], current_index: 0 },
        data: TestData { index: 0, data: vec![TestBucket, TestBucket, TestBucket] },
        next_ctrl: 0,
        end: 9,
    };

    let result = unsafe { iterator.next_impl::<true>() };
    assert!(result.is_some());  

    // Testing for when next_ctrl would be on bound
    iterator.next_ctrl = 9; // setting next_ctrl to end
    let result_end = unsafe { iterator.next_impl::<true>() };
    assert!(result_end.is_none());
}


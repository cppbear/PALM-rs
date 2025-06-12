// Answer 0

#[test]
fn test_set_range_valid_case() {
    struct TestStruct([bool; 10]);
    impl TestStruct {
        fn set_range(&mut self, start: u8, end: u8) {
            debug_assert!(start <= end);
            if start > 0 {
                self.0[start as usize - 1] = true;
            }
            self.0[end as usize] = true;
        }
    }

    let mut test_struct = TestStruct([false; 10]);
    test_struct.set_range(1, 5);
    assert!(test_struct.0[0]); // 1 - 1 = 0 should be true
    assert!(test_struct.0[5]); // 5 should be true
}

#[test]
#[should_panic]
fn test_set_range_start_greater_than_end() {
    struct TestStruct([bool; 10]);
    impl TestStruct {
        fn set_range(&mut self, start: u8, end: u8) {
            debug_assert!(start <= end);
            if start > 0 {
                self.0[start as usize - 1] = true;
            }
            self.0[end as usize] = true;
        }
    }

    let mut test_struct = TestStruct([false; 10]);
    test_struct.set_range(5, 4); // This should trigger a panic since 5 > 4
}

#[test]
#[should_panic]
fn test_set_range_start_equal_end() {
    struct TestStruct([bool; 10]);
    impl TestStruct {
        fn set_range(&mut self, start: u8, end: u8) {
            debug_assert!(start <= end);
            if start > 0 {
                self.0[start as usize - 1] = true;
            }
            self.0[end as usize] = true;
        }
    }

    let mut test_struct = TestStruct([false; 10]);
    test_struct.set_range(0, 0); // This should trigger a panic because start should be > 0
}


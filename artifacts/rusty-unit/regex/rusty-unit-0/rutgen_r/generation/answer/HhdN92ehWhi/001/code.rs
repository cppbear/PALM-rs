// Answer 0

#[test]
fn test_caps_valid_case() {
    struct TestStruct {
        caps: Vec<Option<usize>>,
        slots_per_thread: usize,
    }

    impl TestStruct {
        fn new(slots_per_thread: usize, total_slots: usize) -> Self {
            TestStruct {
                caps: vec![None; total_slots],
                slots_per_thread,
            }
        }

        fn caps(&mut self, pc: usize) -> &mut [Option<usize>] {
            let i = pc * self.slots_per_thread;
            &mut self.caps[i..i + self.slots_per_thread]
        }
    }

    let mut test_struct = TestStruct::new(2, 10);
    let result = test_struct.caps(3);
    assert_eq!(result.len(), 2);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_caps_out_of_bounds() {
    struct TestStruct {
        caps: Vec<Option<usize>>,
        slots_per_thread: usize,
    }

    impl TestStruct {
        fn new(slots_per_thread: usize, total_slots: usize) -> Self {
            TestStruct {
                caps: vec![None; total_slots],
                slots_per_thread,
            }
        }

        fn caps(&mut self, pc: usize) -> &mut [Option<usize>] {
            let i = pc * self.slots_per_thread;
            &mut self.caps[i..i + self.slots_per_thread]
        }
    }

    let mut test_struct = TestStruct::new(3, 9);
    let _result = test_struct.caps(4); // This should panic
}

#[test]
fn test_caps_exact_fit() {
    struct TestStruct {
        caps: Vec<Option<usize>>,
        slots_per_thread: usize,
    }

    impl TestStruct {
        fn new(slots_per_thread: usize, total_slots: usize) -> Self {
            TestStruct {
                caps: vec![None; total_slots],
                slots_per_thread,
            }
        }

        fn caps(&mut self, pc: usize) -> &mut [Option<usize>] {
            let i = pc * self.slots_per_thread;
            &mut self.caps[i..i + self.slots_per_thread]
        }
    }

    let mut test_struct = TestStruct::new(4, 16);
    let result = test_struct.caps(2);
    assert_eq!(result.len(), 4);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_caps_edge_case() {
    struct TestStruct {
        caps: Vec<Option<usize>>,
        slots_per_thread: usize,
    }

    impl TestStruct {
        fn new(slots_per_thread: usize, total_slots: usize) -> Self {
            TestStruct {
                caps: vec![None; total_slots],
                slots_per_thread,
            }
        }

        fn caps(&mut self, pc: usize) -> &mut [Option<usize>] {
            let i = pc * self.slots_per_thread;
            &mut self.caps[i..i + self.slots_per_thread]
        }
    }

    let mut test_struct = TestStruct::new(2, 6);
    let _result = test_struct.caps(3); // This should panic
}


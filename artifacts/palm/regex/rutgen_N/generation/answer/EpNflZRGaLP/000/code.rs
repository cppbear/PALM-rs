// Answer 0

#[test]
fn test_has_visited_new_visit() {
    struct MockMemory {
        visited: Vec<u32>,
    }

    struct TestStruct {
        input: Vec<u8>,
        m: MockMemory,
    }

    impl TestStruct {
        fn new(input: Vec<u8>, bit_size: usize) -> Self {
            let visited_length = ((input.len() + 1) * bit_size + 31) / 32;
            Self {
                input,
                m: MockMemory {
                    visited: vec![0; visited_length],
                },
            }
        }

        fn has_visited(&mut self, ip: usize, at: usize) -> bool {
            let k = ip * (self.input.len() + 1) + at;
            let k1 = k / 32;
            let k2 = 1 << (k % 32);
            if self.m.visited[k1] & k2 == 0 {
                self.m.visited[k1] |= k2;
                false
            } else {
                true
            }
        }
    }

    let mut test_struct = TestStruct::new(vec![1, 2, 3], 32);
    assert!(!test_struct.has_visited(0, 0));
    assert!(test_struct.has_visited(0, 0));
}

#[test]
fn test_has_visited_boundary() {
    struct MockMemory {
        visited: Vec<u32>,
    }

    struct TestStruct {
        input: Vec<u8>,
        m: MockMemory,
    }

    impl TestStruct {
        fn new(input: Vec<u8>, bit_size: usize) -> Self {
            let visited_length = ((input.len() + 1) * bit_size + 31) / 32;
            Self {
                input,
                m: MockMemory {
                    visited: vec![0; visited_length],
                },
            }
        }

        fn has_visited(&mut self, ip: usize, at: usize) -> bool {
            let k = ip * (self.input.len() + 1) + at;
            let k1 = k / 32;
            let k2 = 1 << (k % 32);
            if self.m.visited[k1] & k2 == 0 {
                self.m.visited[k1] |= k2;
                false
            } else {
                true
            }
        }
    }

    let mut test_struct = TestStruct::new(vec![1, 2, 3], 32);
    let input_length = test_struct.input.len();
    let last_index = input_length; // should test the "at" position equal to input length

    assert!(!test_struct.has_visited(0, last_index));
    assert!(test_struct.has_visited(0, last_index));
}


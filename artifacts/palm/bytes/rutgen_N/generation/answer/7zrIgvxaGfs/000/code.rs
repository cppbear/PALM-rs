// Answer 0

#[test]
fn test_remaining_empty() {
    struct VecDeque {
        data: Vec<u8>,
    }

    impl VecDeque {
        fn len(&self) -> usize {
            self.data.len()
        }

        fn remaining(&self) -> usize {
            self.len()
        }
    }

    let deque = VecDeque { data: Vec::new() };
    assert_eq!(deque.remaining(), 0);
}

#[test]
fn test_remaining_non_empty() {
    struct VecDeque {
        data: Vec<u8>,
    }

    impl VecDeque {
        fn len(&self) -> usize {
            self.data.len()
        }

        fn remaining(&self) -> usize {
            self.len()
        }
    }

    let deque = VecDeque { data: vec![1, 2, 3] };
    assert_eq!(deque.remaining(), 3);
}

#[test]
fn test_remaining_after_push() {
    struct VecDeque {
        data: Vec<u8>,
    }

    impl VecDeque {
        fn len(&self) -> usize {
            self.data.len()
        }

        fn remaining(&self) -> usize {
            self.len()
        }

        fn push(&mut self, value: u8) {
            self.data.push(value);
        }
    }

    let mut deque = VecDeque { data: Vec::new() };
    deque.push(1);
    deque.push(2);
    assert_eq!(deque.remaining(), 2);
}


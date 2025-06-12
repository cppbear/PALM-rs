// Answer 0

#[test]
fn test_remaining() {
    struct TestBuf {
        deque: VecDeque<u8>,
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.deque.len()
        }

        fn chunk(&self) -> &[u8] {
            &self.deque
        }

        fn advance(&mut self, cnt: usize) {
            self.deque.drain(..cnt);
        }
    }

    let mut buf = TestBuf {
        deque: VecDeque::from(vec![1, 2, 3, 4, 5]),
    };

    assert_eq!(buf.remaining(), 5);
    buf.advance(2);
    assert_eq!(buf.remaining(), 3);
}

#[test]
fn test_remaining_empty() {
    struct TestBuf {
        deque: VecDeque<u8>,
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.deque.len()
        }

        fn chunk(&self) -> &[u8] {
            &self.deque
        }

        fn advance(&mut self, cnt: usize) {
            self.deque.drain(..cnt);
        }
    }

    let mut buf = TestBuf {
        deque: VecDeque::new(),
    };

    assert_eq!(buf.remaining(), 0);
}


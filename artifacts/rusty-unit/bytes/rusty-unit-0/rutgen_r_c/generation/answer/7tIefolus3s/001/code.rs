// Answer 0

#[test]
fn test_put_with_remaining_src() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }

        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn has_remaining(&self) -> bool {
            self.remaining() > 0
        }
    }

    impl super::Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.remaining()
        }

        fn chunk(&self) -> &[u8] {
            self.chunk()
        }

        fn advance(&mut self, cnt: usize) {
            self.advance(cnt);
        }
    }

    let mut vec = Vec::new();
    let test_data = TestBuf::new(vec![1, 2, 3, 4]);

    unsafe {
        test_data.put(test_data);
    }

    assert_eq!(vec, vec![1, 2, 3, 4]);
}

#[test]
#[should_panic]
fn test_put_with_empty_src() {
    struct EmptyBuf;

    impl EmptyBuf {
        fn new() -> Self {
            Self
        }
    }

    impl super::Buf for EmptyBuf {
        fn remaining(&self) -> usize {
            0
        }

        fn chunk(&self) -> &[u8] {
            &[]
        }

        fn advance(&mut self, _cnt: usize) {}
    }

    let mut vec = Vec::new();

    unsafe {
        vec.put(EmptyBuf::new());
    }
}


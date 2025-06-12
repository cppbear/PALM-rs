// Answer 0

#[test]
fn test_remaining_within_limit() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
            todo!()
        }
    }

    let inner_buf = TestBuf { data: vec![1, 2, 3, 4, 5], position: 0 };
    let take_buf = Take { inner: inner_buf, limit: 3 };
    assert_eq!(take_buf.remaining(), 3);
}

#[test]
fn test_remaining_exceeding_limit() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
            todo!()
        }
    }

    let inner_buf = TestBuf { data: vec![1, 2, 3, 4, 5], position: 0 };
    let take_buf = Take { inner: inner_buf, limit: 10 };
    assert_eq!(take_buf.remaining(), 5);
}

#[test]
fn test_remaining_at_zero() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        fn chunk(&self) -> &[u8] {
            &self.data[self.position..]
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn copy_to_bytes(&mut self, len: usize) -> crate::Bytes {
            todo!()
        }
    }

    let inner_buf = TestBuf { data: vec![], position: 0 };
    let take_buf = Take { inner: inner_buf, limit: 0 };
    assert_eq!(take_buf.remaining(), 0);
}


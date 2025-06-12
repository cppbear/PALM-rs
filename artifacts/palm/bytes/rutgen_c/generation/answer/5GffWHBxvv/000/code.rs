// Answer 0

#[test]
fn test_remaining_non_empty() {
    struct TestBuf {
        data: Vec<u8>,
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.data.len()
        }

        fn chunk(&self) -> &[u8] {
            &self.data
        }

        fn advance(&mut self, cnt: usize) {
            self.data.drain(0..cnt);
        }
    }

    let buf = TestBuf { data: vec![1, 2, 3, 4] };
    assert_eq!(buf.remaining(), 4);
}

#[test]
fn test_remaining_empty() {
    struct TestBuf {
        data: Vec<u8>,
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.data.len()
        }

        fn chunk(&self) -> &[u8] {
            &self.data
        }

        fn advance(&mut self, cnt: usize) {
            self.data.drain(0..cnt);
        }
    }

    let buf = TestBuf { data: vec![] };
    assert_eq!(buf.remaining(), 0);
}


// Answer 0

#[test]
fn test_advance_with_remaining_a_greater_than_cnt() {
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
    }

    let buf_a = TestBuf { data: vec![1, 2, 3], position: 0 };
    let buf_b = TestBuf { data: vec![4, 5, 6], position: 0 };
    let mut chain_buf = Chain { a: buf_a, b: buf_b };

    // a_rem (3) >= cnt (2)
    chain_buf.advance(2);
    assert_eq!(chain_buf.a.position, 2);
    assert_eq!(chain_buf.b.position, 0);
}

#[test]
fn test_advance_with_remaining_a_less_than_cnt() {
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
    }

    let buf_a = TestBuf { data: vec![1, 2], position: 0 };
    let buf_b = TestBuf { data: vec![3, 4, 5], position: 0 };
    let mut chain_buf = Chain { a: buf_a, b: buf_b };

    // a_rem (2) < cnt (3)
    chain_buf.advance(3);
    assert_eq!(chain_buf.a.position, 2); // a should be fully advanced
    assert_eq!(chain_buf.b.position, 1); // b should be advanced by 1
} 

#[test]
#[should_panic]
fn test_advance_with_remaining_a_zero() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            0
        }

        fn chunk(&self) -> &[u8] {
            &[]
        }

        fn advance(&mut self, cnt: usize) {
            self.position += cnt;
        }
    }

    let buf_a = TestBuf { data: vec![1], position: 0 };
    let buf_b = TestBuf { data: vec![2, 3, 4], position: 0 };
    let mut chain_buf = Chain { a: buf_a, b: buf_b };

    // This will panic since a's remaining is 0
    chain_buf.advance(1);
}


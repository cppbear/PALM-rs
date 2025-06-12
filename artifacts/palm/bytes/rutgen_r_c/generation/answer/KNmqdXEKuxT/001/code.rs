// Answer 0

#[test]
fn test_last_ref_with_two_different_bufs() {
    struct TestBufA;
    struct TestBufB;

    impl Buf for TestBufA {
        fn remaining(&self) -> usize { 5 }
        fn bytes(&self) -> &[u8] { &b"hello"[..] }
        fn advance(&mut self, _n: usize) {}
    }

    impl Buf for TestBufB {
        fn remaining(&self) -> usize { 5 }
        fn bytes(&self) -> &[u8] { &b"world"[..] }
        fn advance(&mut self, _n: usize) {}
    }

    let buf_a = TestBufA;
    let buf_b = TestBufB;
    let chain = Chain::new(buf_a, buf_b);

    assert_eq!(chain.last_ref().bytes(), b"world"[..]);
}

#[test]
fn test_last_ref_with_empty_buf() {
    struct EmptyBuf;

    impl Buf for EmptyBuf {
        fn remaining(&self) -> usize { 0 }
        fn bytes(&self) -> &[u8] { &[] }
        fn advance(&mut self, _n: usize) {}
    }

    let empty_buf = EmptyBuf;
    let buf_b = TestBufB;

    let chain = Chain::new(empty_buf, buf_b);
    assert_eq!(chain.last_ref().bytes(), b"world"[..]);
}

#[test]
fn test_last_ref_with_identical_bufs() {
    struct IdenticalBuf;

    impl Buf for IdenticalBuf {
        fn remaining(&self) -> usize { 5 }
        fn bytes(&self) -> &[u8] { &b"world"[..] }
        fn advance(&mut self, _n: usize) {}
    }

    let identical_buf_a = IdenticalBuf;
    let identical_buf_b = IdenticalBuf;

    let chain = Chain::new(identical_buf_a, identical_buf_b);
    assert_eq!(chain.last_ref().bytes(), b"world"[..]);
}


// Answer 0

#[test]
fn test_chunk() {
    struct TestBuf<'a>(&'a [u8]);

    impl<'a> Buf for TestBuf<'a> {
        fn remaining(&self) -> usize {
            self.0.len()
        }

        fn chunk(&self) -> &[u8] {
            self.0
        }

        fn advance(&mut self, cnt: usize) {
            self.0 = &self.0[cnt..];
        }
    }

    let data = &[1, 2, 3, 4, 5];
    let buf = TestBuf(data);

    assert_eq!(buf.chunk(), data);
}

#[test]
fn test_chunk_after_advance() {
    struct TestBuf<'a>(&'a [u8]);

    impl<'a> Buf for TestBuf<'a> {
        fn remaining(&self) -> usize {
            self.0.len()
        }

        fn chunk(&self) -> &[u8] {
            self.0
        }

        fn advance(&mut self, cnt: usize) {
            self.0 = &self.0[cnt..];
        }
    }

    let mut buf = TestBuf(&[1, 2, 3, 4, 5]);

    buf.advance(2);
    assert_eq!(buf.chunk(), &[3, 4, 5]);
}


// Answer 0

#[test]
fn test_chain_remaining() {
    struct TestBuf {
        remaining: usize,
    }
    
    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.remaining
        }
        fn chunk(&self) -> &[u8] { &[] }
        fn advance(&mut self, _: usize) {}
        #[cfg(feature = "std")]
        fn chunks_vectored<'a>(&'a self, _: &mut [IoSlice<'a>]) -> usize { 0 }
        fn copy_to_bytes(&mut self, _: usize) -> crate::Bytes { crate::Bytes::new() }
    }

    let buf1 = TestBuf { remaining: 5 };
    let buf2 = TestBuf { remaining: 10 };
    let chained_buf = Chain { a: buf1, b: buf2 };

    assert_eq!(chained_buf.remaining(), 15);
}

#[test]
fn test_chain_remaining_zero() {
    struct TestBuf {
        remaining: usize,
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.remaining
        }
        fn chunk(&self) -> &[u8] { &[] }
        fn advance(&mut self, _: usize) {}
        #[cfg(feature = "std")]
        fn chunks_vectored<'a>(&'a self, _: &mut [IoSlice<'a>]) -> usize { 0 }
        fn copy_to_bytes(&mut self, _: usize) -> crate::Bytes { crate::Bytes::new() }
    }

    let buf1 = TestBuf { remaining: 0 };
    let buf2 = TestBuf { remaining: 0 };
    let chained_buf = Chain { a: buf1, b: buf2 };

    assert_eq!(chained_buf.remaining(), 0);
}

#[test]
fn test_chain_remaining_with_one_zero() {
    struct TestBuf {
        remaining: usize,
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.remaining
        }
        fn chunk(&self) -> &[u8] { &[] }
        fn advance(&mut self, _: usize) {}
        #[cfg(feature = "std")]
        fn chunks_vectored<'a>(&'a self, _: &mut [IoSlice<'a>]) -> usize { 0 }
        fn copy_to_bytes(&mut self, _: usize) -> crate::Bytes { crate::Bytes::new() }
    }

    let buf1 = TestBuf { remaining: 5 };
    let buf2 = TestBuf { remaining: 0 };
    let chained_buf = Chain { a: buf1, b: buf2 };

    assert_eq!(chained_buf.remaining(), 5);
}

#[test]
fn test_chain_remaining_large_numbers() {
    struct TestBuf {
        remaining: usize,
    }

    impl Buf for TestBuf {
        fn remaining(&self) -> usize {
            self.remaining
        }
        fn chunk(&self) -> &[u8] { &[] }
        fn advance(&mut self, _: usize) {}
        #[cfg(feature = "std")]
        fn chunks_vectored<'a>(&'a self, _: &mut [IoSlice<'a>]) -> usize { 0 }
        fn copy_to_bytes(&mut self, _: usize) -> crate::Bytes { crate::Bytes::new() }
    }

    let buf1 = TestBuf { remaining: usize::MAX - 1 };
    let buf2 = TestBuf { remaining: 1 };
    let chained_buf = Chain { a: buf1, b: buf2 };

    assert_eq!(chained_buf.remaining(), usize::MAX);
}


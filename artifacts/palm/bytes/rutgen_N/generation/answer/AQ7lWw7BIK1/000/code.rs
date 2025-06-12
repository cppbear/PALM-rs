// Answer 0

#[test]
fn test_into_inner() {
    struct TestBuf {
        data: Vec<u8>,
        remaining: usize,
    }

    impl TestBuf {
        fn from(data: &[u8]) -> Self {
            Self {
                data: data.to_vec(),
                remaining: data.len(),
            }
        }

        fn remaining(&self) -> usize {
            self.remaining
        }
    }

    struct IntoIter {
        inner: TestBuf,
        index: usize,
    }

    impl IntoIter {
        fn new(buf: TestBuf) -> Self {
            Self { inner: buf, index: 0 }
        }

        fn next(&mut self) -> Option<u8> {
            if self.index < self.inner.data.len() {
                let item = self.inner.data[self.index];
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }

        fn into_inner(self) -> TestBuf {
            self.inner
        }
    }

    let buf = TestBuf::from(&b"abc"[..]);
    let mut iter = IntoIter::new(buf);

    assert_eq!(iter.next(), Some(b'a'));
    
    let buf = iter.into_inner();
    assert_eq!(2, buf.remaining());
}


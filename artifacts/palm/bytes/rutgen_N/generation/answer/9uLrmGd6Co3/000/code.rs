// Answer 0

#[cfg(test)]
mod tests {
    use bytes::{Buf, Bytes};

    struct MyBuf {
        inner: Bytes,
    }

    impl MyBuf {
        fn new(data: &[u8]) -> Self {
            MyBuf {
                inner: Bytes::copy_from_slice(data),
            }
        }
    }

    #[test]
    fn test_get_ref() {
        let data = b"abc";
        let buf = MyBuf::new(data);
        let iter = buf.inner.clone().into_iter();

        assert_eq!(iter.next(), Some(b'a'));

        // Ensure we are getting the correct remaining bytes
        assert_eq!(buf.inner.remaining(), 3); // Initially 3 bytes
    }

    #[test]
    fn test_get_ref_after_consuming_iter() {
        let data = b"xyz";
        let buf = MyBuf::new(data);
        let mut iter = buf.inner.clone().into_iter();

        // Consume two elements
        iter.next();
        iter.next();

        assert_eq!(buf.inner.remaining(), 3); // Still 3 bytes in the original buffer
    }

    #[test]
    fn test_get_ref_empty_buf() {
        let data: &[u8] = &[];
        let buf = MyBuf::new(data);
        let iter = buf.inner.clone().into_iter();

        assert_eq!(iter.next(), None);
        assert_eq!(buf.inner.remaining(), 0); // No bytes remaining
    }
}


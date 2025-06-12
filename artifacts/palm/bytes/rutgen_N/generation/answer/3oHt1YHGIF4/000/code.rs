// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;

    struct IntoIter<T> {
        inner: T,
    }

    impl<'a> Iterator for IntoIter<&'a [u8]> {
        type Item = u8;

        fn next(&mut self) -> Option<Self::Item> {
            if self.inner.is_empty() {
                None
            } else {
                let first = self.inner[0];
                self.inner = &self.inner[1..];
                Some(first)
            }
        }
    }

    #[test]
    fn test_byte_iterator() {
        let buf = Bytes::from_static(b"abc");
        let mut iter = IntoIter { inner: buf.as_ref() };

        assert_eq!(iter.next(), Some(b'a'));
        assert_eq!(iter.next(), Some(b'b'));
        assert_eq!(iter.next(), Some(b'c'));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_empty_byte_iterator() {
        let buf = Bytes::from_static(b"");
        let mut iter = IntoIter { inner: buf.as_ref() };

        assert_eq!(iter.next(), None);
    }
}


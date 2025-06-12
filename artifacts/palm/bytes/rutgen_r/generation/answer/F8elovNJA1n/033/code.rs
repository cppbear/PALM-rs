// Answer 0

#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use std::ops::Bound;

    #[test]
    fn test_slice_valid_range() {
        let a = Bytes::from(&b"hello world"[..]);
        let b = a.slice(2..5);
        assert_eq!(&b[..], b"llo");
    }

    #[test]
    fn test_slice_empty_result() {
        let a = Bytes::from(&b"hello world"[..]);
        let b = a.slice(5..5);
        assert!(b.is_empty());
    }

    #[should_panic(expected = "range start must not be greater than end")]
    #[test]
    fn test_slice_panic_due_to_begin_greater_than_end() {
        let a = Bytes::from(&b"hello world"[..]);
        let _ = a.slice(5..2);
    }

    #[should_panic(expected = "range end out of bounds:")]
    #[test]
    fn test_slice_panic_due_to_end_out_of_bounds() {
        let a = Bytes::from(&b"hello world"[..]);
        let _ = a.slice(0..12);
    }

    #[should_panic(expected = "out of range")]
    #[test]
    fn test_slice_panic_due_to_checked_add_overflow() {
        let a = Bytes::from(&b"hello world"[..]);
        let _ = a.slice(usize::MAX..=usize::MAX);
    }

    #[test]
    fn test_slice_unbounded_start() {
        let a = Bytes::from(&b"hello world"[..]);
        let b = a.slice(..5);
        assert_eq!(&b[..], b"hello");
    }

    #[test]
    fn test_slice_unbounded_end() {
        let a = Bytes::from(&b"hello world"[..]);
        let b = a.slice(6..);
        assert_eq!(&b[..], b"world");
    }
}


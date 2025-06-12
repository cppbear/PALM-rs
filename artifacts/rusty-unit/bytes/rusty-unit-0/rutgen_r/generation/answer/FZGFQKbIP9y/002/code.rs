// Answer 0

#[test]
#[should_panic(expected = "set_len out of bounds")]
fn test_set_len_exceeds_capacity() {
    struct BytesMut {
        cap: usize,
        len: usize,
    }

    impl BytesMut {
        fn from(slice: &[u8]) -> Self {
            BytesMut {
                cap: slice.len(),
                len: slice.len(),
            }
        }

        unsafe fn set_len(&mut self, len: usize) {
            debug_assert!(len <= self.cap, "set_len out of bounds");
            self.len = len;
        }
    }

    let mut b = BytesMut::from(&b"hello"[..]);

    unsafe {
        // Attempt to set a length greater than capacity
        b.set_len(10);
    }
} 

#[test]
fn test_set_len_valid_length() {
    struct BytesMut {
        cap: usize,
        len: usize,
    }

    impl BytesMut {
        fn from(slice: &[u8]) -> Self {
            BytesMut {
                cap: slice.len(),
                len: slice.len(),
            }
        }

        unsafe fn set_len(&mut self, len: usize) {
            debug_assert!(len <= self.cap, "set_len out of bounds");
            self.len = len;
        }
    }

    let mut b = BytesMut::from(&b"hello"[..]);

    unsafe {
        // Set a valid length
        b.set_len(5);
        assert_eq!(b.len, 5);

        // Set back to the original length
        b.set_len(5);
        assert_eq!(b.len, 5);
    }
}

#[test]
#[should_panic(expected = "set_len out of bounds")]
fn test_set_len_zero_length() {
    struct BytesMut {
        cap: usize,
        len: usize,
    }

    impl BytesMut {
        fn from(slice: &[u8]) -> Self {
            BytesMut {
                cap: slice.len(),
                len: slice.len(),
            }
        }

        unsafe fn set_len(&mut self, len: usize) {
            debug_assert!(len <= self.cap, "set_len out of bounds");
            self.len = len;
        }
    }

    let mut b = BytesMut::from(&b"hello"[..]);

    unsafe {
        // Attempt to set length to 0, which is valid for this context
        b.set_len(0);
        assert_eq!(b.len, 0);
        
        // Set length beyond initial capacity
        b.set_len(6); // This should panic
    }
}


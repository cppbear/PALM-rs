// Answer 0

#[test]
fn test_as_str_valid_utf8() {
    struct BytesContainer {
        bytes: Vec<u8>,
        offset: usize,
    }

    impl BytesContainer {
        pub fn as_str(&self) -> &str {
            let slice = &self.bytes[..self.offset];
            unsafe { std::str::from_utf8_unchecked(slice) }
        }
    }

    let valid_bytes = vec![72, 101, 108, 108, 111]; // Hello
    let container = BytesContainer {
        bytes: valid_bytes,
        offset: 5,
    };

    assert_eq!(container.as_str(), "Hello");
}

#[test]
#[should_panic]
fn test_as_str_offset_out_of_bounds() {
    struct BytesContainer {
        bytes: Vec<u8>,
        offset: usize,
    }

    impl BytesContainer {
        pub fn as_str(&self) -> &str {
            let slice = &self.bytes[..self.offset];
            unsafe { std::str::from_utf8_unchecked(slice) }
        }
    }

    let valid_bytes = vec![72, 101, 108, 108, 111]; // Hello
    let container = BytesContainer {
        bytes: valid_bytes,
        offset: 6, // out of bounds
    };

    let _result = container.as_str(); // This should panic
}

#[test]
#[should_panic]
fn test_as_str_negative_offset() {
    struct BytesContainer {
        bytes: Vec<u8>,
        offset: usize,
    }

    impl BytesContainer {
        pub fn as_str(&self) -> &str {
            let slice = &self.bytes[..self.offset];
            unsafe { std::str::from_utf8_unchecked(slice) }
        }
    }

    let valid_bytes = vec![72, 101, 108, 108, 111]; // Hello
    let container = BytesContainer {
        bytes: valid_bytes,
        offset: usize::MAX, // negative offset simulated with overflow
    };

    let _result = container.as_str(); // This should panic
}

#[test]
fn test_as_str_zero_length() {
    struct BytesContainer {
        bytes: Vec<u8>,
        offset: usize,
    }

    impl BytesContainer {
        pub fn as_str(&self) -> &str {
            let slice = &self.bytes[..self.offset];
            unsafe { std::str::from_utf8_unchecked(slice) }
        }
    }

    let valid_bytes = vec![72, 101, 108, 108, 111]; // Hello
    let container = BytesContainer {
        bytes: valid_bytes,
        offset: 0, // zero length
    };

    assert_eq!(container.as_str(), ""); // should return empty string
}


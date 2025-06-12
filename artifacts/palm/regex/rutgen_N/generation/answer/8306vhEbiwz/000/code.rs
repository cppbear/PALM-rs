// Answer 0

#[test]
fn test_as_bytes_valid_range() {
    struct TestText {
        text: Vec<u8>,
        start: usize,
        end: usize,
    }

    impl TestText {
        pub fn as_bytes(&self) -> &[u8] {
            &self.text[self.start..self.end]
        }
    }

    let test_instance = TestText {
        text: b"Hello, world!".to_vec(),
        start: 0,
        end: 5,
    };

    let result = test_instance.as_bytes();
    assert_eq!(result, b"Hello");
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_as_bytes_start_out_of_bounds() {
    struct TestText {
        text: Vec<u8>,
        start: usize,
        end: usize,
    }

    impl TestText {
        pub fn as_bytes(&self) -> &[u8] {
            &self.text[self.start..self.end]
        }
    }

    let test_instance = TestText {
        text: b"Hello, world!".to_vec(),
        start: 20, // Out of bounds
        end: 25,
    };

    let _ = test_instance.as_bytes();
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_as_bytes_end_out_of_bounds() {
    struct TestText {
        text: Vec<u8>,
        start: usize,
        end: usize,
    }

    impl TestText {
        pub fn as_bytes(&self) -> &[u8] {
            &self.text[self.start..self.end]
        }
    }

    let test_instance = TestText {
        text: b"Hello, world!".to_vec(),
        start: 5,
        end: 20, // Out of bounds
    };

    let _ = test_instance.as_bytes();
}

#[test]
fn test_as_bytes_empty_slice() {
    struct TestText {
        text: Vec<u8>,
        start: usize,
        end: usize,
    }

    impl TestText {
        pub fn as_bytes(&self) -> &[u8] {
            &self.text[self.start..self.end]
        }
    }

    let test_instance = TestText {
        text: b"Hello, world!".to_vec(),
        start: 5,
        end: 5, // Empty range
    };

    let result = test_instance.as_bytes();
    assert_eq!(result.len(), 0);
}


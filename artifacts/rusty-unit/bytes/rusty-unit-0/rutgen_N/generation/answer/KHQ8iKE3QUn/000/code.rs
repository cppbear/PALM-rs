// Answer 0

#[test]
fn test_put_slice_empty() {
    struct Buffer {
        data: Vec<u8>,
    }

    impl Buffer {
        fn new() -> Self {
            Buffer { data: Vec::new() }
        }

        fn extend_from_slice(&mut self, src: &[u8]) {
            self.data.extend_from_slice(src);
        }

        fn put_slice(&mut self, src: &[u8]) {
            self.extend_from_slice(src);
        }

        fn get_data(&self) -> &[u8] {
            &self.data
        }
    }

    let mut buffer = Buffer::new();
    buffer.put_slice(&[]);
    assert_eq!(buffer.get_data(), &[]);
}

#[test]
fn test_put_slice_non_empty() {
    struct Buffer {
        data: Vec<u8>,
    }

    impl Buffer {
        fn new() -> Self {
            Buffer { data: Vec::new() }
        }

        fn extend_from_slice(&mut self, src: &[u8]) {
            self.data.extend_from_slice(src);
        }

        fn put_slice(&mut self, src: &[u8]) {
            self.extend_from_slice(src);
        }

        fn get_data(&self) -> &[u8] {
            &self.data
        }
    }

    let mut buffer = Buffer::new();
    buffer.put_slice(&[1, 2, 3]);
    assert_eq!(buffer.get_data(), &[1, 2, 3]);
}

#[test]
fn test_put_slice_multiple_calls() {
    struct Buffer {
        data: Vec<u8>,
    }

    impl Buffer {
        fn new() -> Self {
            Buffer { data: Vec::new() }
        }

        fn extend_from_slice(&mut self, src: &[u8]) {
            self.data.extend_from_slice(src);
        }

        fn put_slice(&mut self, src: &[u8]) {
            self.extend_from_slice(src);
        }

        fn get_data(&self) -> &[u8] {
            &self.data
        }
    }

    let mut buffer = Buffer::new();
    buffer.put_slice(&[1, 2]);
    buffer.put_slice(&[3, 4]);
    assert_eq!(buffer.get_data(), &[1, 2, 3, 4]);
}

#[test]
fn test_put_slice_edge_case() {
    struct Buffer {
        data: Vec<u8>,
    }

    impl Buffer {
        fn new() -> Self {
            Buffer { data: Vec::new() }
        }

        fn extend_from_slice(&mut self, src: &[u8]) {
            self.data.extend_from_slice(src);
        }

        fn put_slice(&mut self, src: &[u8]) {
            self.extend_from_slice(src);
        }

        fn get_data(&self) -> &[u8] {
            &self.data
        }
    }

    let mut buffer = Buffer::new();
    buffer.put_slice(&[255]); // Testing upper boundary byte value
    assert_eq!(buffer.get_data(), &[255]);
}


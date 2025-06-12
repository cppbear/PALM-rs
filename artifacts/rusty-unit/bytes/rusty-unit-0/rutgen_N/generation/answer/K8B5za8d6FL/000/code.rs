// Answer 0

#[test]
fn test_chunk_valid_position() {
    struct Buffer {
        data: Vec<u8>,
        pos: usize,
    }

    impl Buffer {
        fn new(data: Vec<u8>, pos: usize) -> Self {
            Buffer { data, pos }
        }
        
        fn get_ref(&self) -> &[u8] {
            &self.data
        }
        
        fn position(&self) -> usize {
            self.pos
        }
    }

    let buffer = Buffer::new(vec![1, 2, 3, 4, 5], 2);
    let chunk = buffer.chunk();
    assert_eq!(chunk, &[3, 4, 5]);
}

#[test]
fn test_chunk_zero_position() {
    struct Buffer {
        data: Vec<u8>,
        pos: usize,
    }

    impl Buffer {
        fn new(data: Vec<u8>, pos: usize) -> Self {
            Buffer { data, pos }
        }
        
        fn get_ref(&self) -> &[u8] {
            &self.data
        }
        
        fn position(&self) -> usize {
            self.pos
        }
    }

    let buffer = Buffer::new(vec![1, 2, 3, 4, 5], 0);
    let chunk = buffer.chunk();
    assert_eq!(chunk, &[1, 2, 3, 4, 5]);
}

#[test]
fn test_chunk_position_beyond_length() {
    struct Buffer {
        data: Vec<u8>,
        pos: usize,
    }

    impl Buffer {
        fn new(data: Vec<u8>, pos: usize) -> Self {
            Buffer { data, pos }
        }
        
        fn get_ref(&self) -> &[u8] {
            &self.data
        }
        
        fn position(&self) -> usize {
            self.pos
        }
    }

    let buffer = Buffer::new(vec![1, 2, 3, 4, 5], 10);
    let chunk = buffer.chunk();
    assert_eq!(chunk, &[]);
}


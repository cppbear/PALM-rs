// Answer 0

#[derive(Debug)]
struct MockBuf {
    data: Vec<u8>,
}

struct BufReader {
    buf: MockBuf,
}

impl BufReader {
    fn new(data: Vec<u8>) -> Self {
        Self {
            buf: MockBuf { data },
        }
    }

    pub fn get_mut(&mut self) -> &mut MockBuf {
        &mut self.buf
    }
}

#[test]
fn test_get_mut() {
    let mut reader = BufReader::new(vec![1, 2, 3]);

    // Get mutable reference to the underlying buffer
    let buf_ref = reader.get_mut();
    
    // Modify the buffer
    buf_ref.data.push(4);
    
    // Verify the buffer was modified
    assert_eq!(buf_ref.data, vec![1, 2, 3, 4]);
}

#[test]
fn test_get_mut_empty() {
    let mut reader = BufReader::new(vec![]);

    // Get mutable reference to the underlying buffer
    let buf_ref = reader.get_mut();
    
    // Verify the buffer is empty
    assert!(buf_ref.data.is_empty());
}


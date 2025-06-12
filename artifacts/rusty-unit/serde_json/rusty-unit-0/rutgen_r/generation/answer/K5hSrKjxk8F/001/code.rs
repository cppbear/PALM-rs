// Answer 0

#[derive(Default)]
struct MockReader {
    buffer: Vec<u8>,
    position: usize,
}

impl MockReader {
    fn new(buffer: Vec<u8>) -> Self {
        Self { buffer, position: 0 }
    }
    
    fn next(&mut self) -> Result<Option<u8>, &'static str> {
        if self.position < self.buffer.len() {
            let byte = self.buffer[self.position];
            self.position += 1;
            Ok(Some(byte))
        } else {
            Ok(None)
        }
    }
}

struct TestStruct {
    read: MockReader,
}

impl TestStruct {
    fn next_char(&mut self) -> Result<Option<u8>, &'static str> {
        self.read.next()
    }
}

#[test]
fn test_next_char_with_valid_data() {
    let mut test_struct = TestStruct {
        read: MockReader::new(vec![97, 98, 99]), // corresponds to 'a', 'b', 'c'
    };

    assert_eq!(test_struct.next_char().unwrap(), Some(97));
    assert_eq!(test_struct.next_char().unwrap(), Some(98));
    assert_eq!(test_struct.next_char().unwrap(), Some(99));
    assert_eq!(test_struct.next_char().unwrap(), None);
}

#[test]
fn test_next_char_with_empty_data() {
    let mut test_struct = TestStruct {
        read: MockReader::new(vec![]),
    };

    assert_eq!(test_struct.next_char().unwrap(), None);
}

#[test]
#[should_panic]
fn test_next_char_panic_on_invalid_result() {
    let mut test_struct = TestStruct {
        read: MockReader::new(vec![97, 98, 99]),
    };

    test_struct.read.position = 10; // Force position out of bounds
    test_struct.next_char().unwrap(); // This should panic
}


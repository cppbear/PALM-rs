// Answer 0

#[derive(Debug)]
struct MockReader {
    data: Vec<u8>,
    index: usize,
}

impl MockReader {
    fn new(data: Vec<u8>) -> Self {
        MockReader { data, index: 0 }
    }
}

impl Iterator for MockReader {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.data.len() {
            let byte = self.data[self.index];
            self.index += 1;
            Some(byte)
        } else {
            None
        }
    }
}

struct Serializer {
    read: MockReader,
}

impl Serializer {
    fn next_char(&mut self) -> Result<Option<u8>, &'static str> {
        self.read.next().ok_or("End of input")
    }
}

#[test]
fn test_next_char_with_data() {
    let data = vec![104, 101, 108, 108, 111]; // Represents the string "hello"
    let mut serializer = Serializer {
        read: MockReader::new(data),
    };
    
    assert_eq!(serializer.next_char().unwrap(), Some(104)); // 'h'
    assert_eq!(serializer.next_char().unwrap(), Some(101)); // 'e'
    assert_eq!(serializer.next_char().unwrap(), Some(108)); // 'l'
    assert_eq!(serializer.next_char().unwrap(), Some(108)); // 'l'
    assert_eq!(serializer.next_char().unwrap(), Some(111)); // 'o'
    assert_eq!(serializer.next_char(), Err("End of input")); // No more data
}

#[test]
fn test_next_char_empty() {
    let mut serializer = Serializer {
        read: MockReader::new(vec![]),
    };
    
    assert_eq!(serializer.next_char(), Err("End of input")); // No data
}


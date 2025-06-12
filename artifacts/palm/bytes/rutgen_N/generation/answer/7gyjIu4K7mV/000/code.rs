// Answer 0

#[derive(Debug)]
struct Bytes {
    data: Vec<u8>,
}

impl Bytes {
    fn new(data: Vec<u8>) -> Self {
        Bytes { data }
    }

    fn split_to(&mut self, len: usize) -> Self {
        let split_data = self.data.split_off(len);
        split_data.insert(0, self.data.clone());
        self.data.truncate(len);
        Bytes::new(split_data)
    }

    fn copy_to_bytes(&mut self, len: usize) -> Self {
        self.split_to(len)
    }
}

#[test]
fn test_copy_to_bytes_basic() {
    let mut bytes = Bytes::new(vec![1, 2, 3, 4, 5]);
    let copied = bytes.copy_to_bytes(3);
    assert_eq!(bytes.data, vec![1, 2, 3]);
    assert_eq!(copied.data, vec![4, 5]);
}

#[test]
fn test_copy_to_bytes_empty() {
    let mut bytes = Bytes::new(vec![]);
    let copied = bytes.copy_to_bytes(0);
    assert_eq!(bytes.data, vec![]);
    assert_eq!(copied.data, vec![]);
}

#[test]
fn test_copy_to_bytes_exceeding_length() {
    let mut bytes = Bytes::new(vec![1, 2, 3]);
    let copied = bytes.copy_to_bytes(5);
    assert_eq!(bytes.data, vec![1, 2, 3]);
    assert_eq!(copied.data, vec![]);
}

#[test]
fn test_copy_to_bytes_exact_length() {
    let mut bytes = Bytes::new(vec![1, 2, 3]);
    let copied = bytes.copy_to_bytes(3);
    assert_eq!(bytes.data, vec![]);
    assert_eq!(copied.data, vec![1, 2, 3]);
}


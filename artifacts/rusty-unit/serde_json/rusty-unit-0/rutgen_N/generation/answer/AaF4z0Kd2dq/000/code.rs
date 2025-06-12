// Answer 0

#[derive(Debug)]
struct SliceReader {
    slice: Vec<u8>,
    index: usize,
}

impl SliceReader {
    fn new(slice: Vec<u8>) -> Self {
        SliceReader { slice, index: 0 }
    }

    fn next(&mut self) -> Result<Option<u8>, std::io::Error> {
        Ok(if self.index < self.slice.len() {
            let ch = self.slice[self.index];
            self.index += 1;
            Some(ch)
        } else {
            None
        })
    }
}

#[test]
fn test_next_with_elements() {
    let mut reader = SliceReader::new(vec![1, 2, 3]);
    assert_eq!(reader.next().unwrap(), Some(1));
    assert_eq!(reader.next().unwrap(), Some(2));
    assert_eq!(reader.next().unwrap(), Some(3));
    assert_eq!(reader.next().unwrap(), None);
}

#[test]
fn test_next_with_empty_slice() {
    let mut reader = SliceReader::new(vec![]);
    assert_eq!(reader.next().unwrap(), None);
}

#[test]
fn test_next_boundary_conditions() {
    let mut reader = SliceReader::new(vec![10, 20, 30]);
    assert_eq!(reader.next().unwrap(), Some(10));
    assert_eq!(reader.next().unwrap(), Some(20));
    assert_eq!(reader.next().unwrap(), Some(30));
    assert_eq!(reader.next().unwrap(), None);
}


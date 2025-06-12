// Answer 0

#[derive(Debug)]
struct TestStruct {
    ch: Option<char>,
    iter: Iter,
}

struct Iter {
    byte_offset_value: usize,
}

impl Iter {
    fn byte_offset(&self) -> usize {
        self.byte_offset_value
    }
}

impl TestStruct {
    fn byte_offset(&self) -> usize {
        match self.ch {
            Some(_) => self.iter.byte_offset() - 1,
            None => self.iter.byte_offset(),
        }
    }
}

#[test]
fn test_byte_offset_none() {
    let iter = Iter { byte_offset_value: 10 };
    let test_struct = TestStruct { ch: None, iter };

    assert_eq!(test_struct.byte_offset(), 10);
}

#[test]
fn test_byte_offset_none_boundary() {
    let iter = Iter { byte_offset_value: 0 };
    let test_struct = TestStruct { ch: None, iter };

    assert_eq!(test_struct.byte_offset(), 0);
}


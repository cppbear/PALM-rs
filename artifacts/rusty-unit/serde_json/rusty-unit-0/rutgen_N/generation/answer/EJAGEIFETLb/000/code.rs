// Answer 0

#[derive(Default)]
struct Iter {
    offset: usize,
}

impl Iter {
    fn byte_offset(&self) -> usize {
        self.offset
    }
}

struct MyStruct {
    ch: Option<char>,
    iter: Iter,
}

impl MyStruct {
    fn new(ch: Option<char>, offset: usize) -> Self {
        Self {
            ch,
            iter: Iter { offset },
        }
    }

    fn byte_offset(&self) -> usize {
        match self.ch {
            Some(_) => self.iter.byte_offset() - 1,
            None => self.iter.byte_offset(),
        }
    }
}

#[test]
fn test_byte_offset_with_some_char() {
    let my_struct = MyStruct::new(Some('a'), 10);
    assert_eq!(my_struct.byte_offset(), 9);
}

#[test]
fn test_byte_offset_with_none_char() {
    let my_struct = MyStruct::new(None, 10);
    assert_eq!(my_struct.byte_offset(), 10);
}

#[test]
fn test_byte_offset_with_zero_offset_and_some_char() {
    let my_struct = MyStruct::new(Some('a'), 0);
    assert_eq!(my_struct.byte_offset(), usize::MAX);
}

#[test]
fn test_byte_offset_with_zero_offset_and_none_char() {
    let my_struct = MyStruct::new(None, 0);
    assert_eq!(my_struct.byte_offset(), 0);
}


// Answer 0

#[derive(Default)]
struct CharLenStruct {
    char_len: usize,
}

impl CharLenStruct {
    pub fn new(char_len: usize) -> Self {
        CharLenStruct { char_len }
    }
}

impl CharLenStruct {
    pub fn char_len(&self) -> usize {
        self.char_len
    }
}

#[test]
fn test_char_len_zero() {
    let instance = CharLenStruct::new(0);
    assert_eq!(instance.char_len(), 0);
}

#[test]
fn test_char_len_positive() {
    let instance = CharLenStruct::new(5);
    assert_eq!(instance.char_len(), 5);
}

#[test]
fn test_char_len_large() {
    let instance = CharLenStruct::new(usize::MAX);
    assert_eq!(instance.char_len(), usize::MAX);
}


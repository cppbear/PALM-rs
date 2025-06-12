// Answer 0

#[derive(Debug)]
struct TestStruct {
    char_len: usize,
}

impl TestStruct {
    pub fn char_len(&self) -> usize {
        self.char_len
    }
}

#[test]
fn test_char_len_zero() {
    let test_instance = TestStruct { char_len: 0 };
    assert_eq!(test_instance.char_len(), 0);
}

#[test]
fn test_char_len_positive() {
    let test_instance = TestStruct { char_len: 5 };
    assert_eq!(test_instance.char_len(), 5);
}

#[test]
fn test_char_len_large_number() {
    let test_instance = TestStruct { char_len: 1000 };
    assert_eq!(test_instance.char_len(), 1000);
}


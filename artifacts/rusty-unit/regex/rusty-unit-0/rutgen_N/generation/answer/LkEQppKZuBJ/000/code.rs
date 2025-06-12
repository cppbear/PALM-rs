// Answer 0

#[derive(Debug)]
struct MyString(String);

impl MyString {
    fn len(&self) -> usize {
        self.0.len()
    }
}

#[test]
fn test_len_empty_string() {
    let s = MyString("".to_string());
    assert_eq!(s.len(), 0);
}

#[test]
fn test_len_non_empty_string() {
    let s = MyString("Hello".to_string());
    assert_eq!(s.len(), 5);
}

#[test]
fn test_len_whitespace_string() {
    let s = MyString("   ".to_string());
    assert_eq!(s.len(), 3);
}


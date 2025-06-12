// Answer 0

#[derive(Debug)]
struct MyString(String);

impl std::ops::Deref for MyString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[test]
fn test_len() {
    let my_string = MyString(String::from("hello"));
    assert_eq!(my_string.len(), 5);
}

#[test]
fn test_empty_len() {
    let my_string = MyString(String::from(""));
    assert_eq!(my_string.len(), 0);
}


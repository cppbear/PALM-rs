// Answer 0

#[derive(Debug)]
struct MyStruct {
    start: char,
}

impl MyStruct {
    fn lower(&self) -> char {
        self.start
    }
}

#[test]
fn test_lower() {
    let my_instance = MyStruct { start: 'a' };
    assert_eq!(my_instance.lower(), 'a');
}

#[test]
fn test_lower_boundary() {
    let my_instance = MyStruct { start: 'z' };
    assert_eq!(my_instance.lower(), 'z');
}

#[test]
fn test_lower_special_characters() {
    let my_instance = MyStruct { start: '!' };
    assert_eq!(my_instance.lower(), '!');
}


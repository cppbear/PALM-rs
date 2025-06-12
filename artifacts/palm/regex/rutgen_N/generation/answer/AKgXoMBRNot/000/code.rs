// Answer 0

#[derive(Debug)]
struct MyStruct {
    start: char,
}

impl MyStruct {
    fn set_lower(&mut self, bound: char) {
        self.start = bound;
    }
}

#[test]
fn test_set_lower_updates_bound() {
    let mut my_struct = MyStruct { start: 'A' };

    my_struct.set_lower('a');
    
    assert_eq!(my_struct.start, 'a');
}

#[test]
fn test_set_lower_with_same_char() {
    let mut my_struct = MyStruct { start: 'b' };

    my_struct.set_lower('b');
    
    assert_eq!(my_struct.start, 'b');
}

#[test]
fn test_set_lower_changes_to_different_char() {
    let mut my_struct = MyStruct { start: 'C' };

    my_struct.set_lower('d');
    
    assert_eq!(my_struct.start, 'd');
}


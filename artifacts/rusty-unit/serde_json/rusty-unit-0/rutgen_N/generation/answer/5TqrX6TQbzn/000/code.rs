// Answer 0

#[derive(Default)]
struct Reader {
    discarded: bool,
}

impl Reader {
    fn discard(&mut self) {
        self.discarded = true;
    }
}

struct MyStruct {
    read: Reader,
}

impl MyStruct {
    fn eat_char(&mut self) {
        self.read.discard();
    }
}

#[test]
fn test_eat_char() {
    let mut my_struct = MyStruct::default();
    assert!(!my_struct.read.discarded);
    my_struct.eat_char();
    assert!(my_struct.read.discarded);
}

#[test]
fn test_eat_char_multiple_calls() {
    let mut my_struct = MyStruct::default();
    my_struct.eat_char();
    assert!(my_struct.read.discarded);
    
    my_struct.eat_char(); // Calling again does not change state
    assert!(my_struct.read.discarded);
}


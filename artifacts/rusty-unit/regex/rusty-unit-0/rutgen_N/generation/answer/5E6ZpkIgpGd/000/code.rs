// Answer 0

#[derive(Debug)]
struct InputAt {
    content: String,
    position: usize,
}

impl InputAt {
    fn new(content: &str, position: usize) -> Self {
        InputAt {
            content: content.to_string(),
            position,
        }
    }

    fn char(&self) -> char {
        self.content.chars().nth(self.position).unwrap_or('\0')
    }
}

struct MyStruct;

impl MyStruct {
    fn next_char(&self, at: InputAt) -> char {
        at.char()
    }
}

#[test]
fn test_next_char_valid() {
    let input = InputAt::new("hello", 1);
    let my_struct = MyStruct;
    let result = my_struct.next_char(input);
    assert_eq!(result, 'e');
}

#[test]
fn test_next_char_boundary() {
    let input = InputAt::new("test", 0);
    let my_struct = MyStruct;
    let result = my_struct.next_char(input);
    assert_eq!(result, 't');
}

#[test]
fn test_next_char_out_of_bounds() {
    let input = InputAt::new("world", 10);
    let my_struct = MyStruct;
    let result = my_struct.next_char(input);
    assert_eq!(result, '\0');
}


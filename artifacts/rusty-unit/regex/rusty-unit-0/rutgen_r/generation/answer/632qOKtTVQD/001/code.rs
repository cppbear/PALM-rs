// Answer 0

#[derive(Debug)]
struct TestInput {
    data: String,
}

impl TestInput {
    fn next_char(&self, at: usize) -> Option<char> {
        self.data.chars().nth(at)
    }
}

#[test]
fn test_next_char_valid_index() {
    let input = TestInput {
        data: "hello".to_string(),
    };
    assert_eq!(input.next_char(0), Some('h'));
    assert_eq!(input.next_char(1), Some('e'));
    assert_eq!(input.next_char(2), Some('l'));
    assert_eq!(input.next_char(3), Some('l'));
    assert_eq!(input.next_char(4), Some('o'));
}

#[test]
#[should_panic]
fn test_next_char_out_of_bounds() {
    let input = TestInput {
        data: "hello".to_string(),
    };
    let _ = input.next_char(5); // this should panic as 5 is out of bounds
}

#[test]
fn test_next_char_empty_string() {
    let input = TestInput {
        data: "".to_string(),
    };
    assert_eq!(input.next_char(0), None); // no character exists
}

#[test]
fn test_next_char_single_character() {
    let input = TestInput {
        data: "a".to_string(),
    };
    assert_eq!(input.next_char(0), Some('a'));
    assert_eq!(input.next_char(1), None); // there is no second character
}


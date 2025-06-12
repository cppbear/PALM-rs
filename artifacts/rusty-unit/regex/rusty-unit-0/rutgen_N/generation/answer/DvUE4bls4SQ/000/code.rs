// Answer 0

#[derive(Debug)]
struct Input {
    text: String,
}

impl Input {
    fn new(text: &str) -> Self {
        Input {
            text: text.to_string(),
        }
    }

    fn len(&self) -> usize {
        self.text.len()
    }
}

#[test]
fn test_len_empty_string() {
    let input = Input::new("");
    assert_eq!(input.len(), 0);
}

#[test]
fn test_len_non_empty_string() {
    let input = Input::new("Hello, World!");
    assert_eq!(input.len(), 13);
}

#[test]
fn test_len_whitespace_string() {
    let input = Input::new("   ");
    assert_eq!(input.len(), 3);
}

#[test]
fn test_len_long_string() {
    let long_text = "Lorem ipsum dolor sit amet, consectetur adipiscing elit.";
    let input = Input::new(long_text);
    assert_eq!(input.len(), long_text.len());
}


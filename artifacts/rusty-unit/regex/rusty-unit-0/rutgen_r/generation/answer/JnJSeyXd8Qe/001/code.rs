// Answer 0

#[derive(Debug)]
struct DummyText<R> {
    text: R,
}

impl<R: std::fmt::Debug> DummyText<R> {
    fn new(text: R) -> Self {
        DummyText { text }
    }
    
    fn text(&self) -> &R {
        &self.text
    }
}

struct StructWithText<R>(DummyText<R>);

impl<R> StructWithText<R> {
    pub fn text(&self) -> &'_ R {
        self.0.text()
    }
}

#[test]
fn test_text_non_empty() {
    let dummy = DummyText::new(String::from("test text"));
    let struct_with_text = StructWithText(dummy);
    assert_eq!(struct_with_text.text(), "test text");
}

#[test]
fn test_text_empty() {
    let dummy = DummyText::new(String::from(""));
    let struct_with_text = StructWithText(dummy);
    assert_eq!(struct_with_text.text(), "");
}

#[test]
fn test_text_special_characters() {
    let dummy = DummyText::new(String::from("!@#$%^&*()"));
    let struct_with_text = StructWithText(dummy);
    assert_eq!(struct_with_text.text(), "!@#$%^&*()");
}

#[test]
fn test_text_long_string() {
    let long_text = "a".repeat(1000);
    let dummy = DummyText::new(long_text.clone());
    let struct_with_text = StructWithText(dummy);
    assert_eq!(struct_with_text.text(), long_text);
}


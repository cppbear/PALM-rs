// Answer 0

#[derive(Debug)]
struct MyRegex {
    text: String,
}

impl MyRegex {
    pub fn new(text: String) -> Self {
        MyRegex { text }
    }
}

trait RegexTrait {
    type Text;
    fn text(&self) -> &'_ Self::Text;
}

impl RegexTrait for MyRegex {
    type Text = String;

    fn text(&self) -> &'_ Self::Text {
        &self.text
    }
}

#[test]
fn test_text() {
    let regex = MyRegex::new("sample text".to_string());
    assert_eq!(regex.text(), "sample text");
}

#[test]
fn test_text_empty() {
    let regex = MyRegex::new("".to_string());
    assert_eq!(regex.text(), "");
}


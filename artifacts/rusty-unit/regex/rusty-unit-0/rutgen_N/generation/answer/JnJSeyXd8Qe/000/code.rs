// Answer 0

#[derive(Debug)]
struct MockR {
    text: String,
}

impl MockR {
    fn text(&self) -> &String {
        &self.text
    }
}

struct Wrapper<'t, R> (R);

impl<'t, R> Wrapper<'t, R> {
    pub fn text(&self) -> &'t R::Text {
        self.0.text()
    }
}

#[test]
fn test_text() {
    let mock_r = MockR { text: String::from("Hello, world!") };
    let wrapper = Wrapper(mock_r);
    assert_eq!(wrapper.text(), "Hello, world!");
}

#[test]
fn test_empty_text() {
    let mock_r = MockR { text: String::from("") };
    let wrapper = Wrapper(mock_r);
    assert_eq!(wrapper.text(), "");
}


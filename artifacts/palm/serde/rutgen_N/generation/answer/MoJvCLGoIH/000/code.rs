// Answer 0

#[derive(Debug)]
struct Content {
    character: char,
}

impl Content {
    fn Char(value: char) -> Self {
        Content { character: value }
    }
}

trait de {
    type Error;
    type Value;
}

struct MyError;

impl de for MyError {
    type Error = MyError;
    type Value = Content;
}

#[test]
fn test_visit_char() {
    let value = 'a';
    let result: Result<Content, MyError> = visit_char(value);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().character, value);
}

fn visit_char<F>(value: char) -> Result<Content, F>
where
    F: de::Error,
{
    Ok(Content::Char(value))
}


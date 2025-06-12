// Answer 0

#[derive(Debug)]
struct MyError;

impl std::fmt::Display for MyError {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Ok(())
    }
}

impl std::error::Error for MyError {}

struct MyVisitor;

impl MyVisitor {
    fn visit_str<E>(&self, v: &str) -> Result<String, E>
    where
        E: std::error::Error,
    {
        Ok(String::from(v))
    }
}

#[test]
fn test_visit_str() {
    let visitor = MyVisitor;
    let result: Result<String, MyError> = visitor.visit_str("test string");
    assert_eq!(result.unwrap(), "test string");
}

#[test]
#[should_panic]
fn test_visit_str_empty() {
    let visitor = MyVisitor;
    let result: Result<String, MyError> = visitor.visit_str("");
    assert_eq!(result.unwrap(), "");
}


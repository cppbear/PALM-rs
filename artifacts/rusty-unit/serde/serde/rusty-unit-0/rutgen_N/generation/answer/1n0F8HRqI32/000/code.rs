// Answer 0

#[derive(Debug)]
struct TestError;

impl serde::de::Error for TestError {
    fn custom<T>(msg: T) -> Self where T: std::fmt::Display {
        TestError
    }
}

struct Visitor;

impl serde::de::Visitor for Visitor {
    type Value = &'static str;

    fn visit_borrowed_str<E>(self, v: &'static str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v)
    }
}

#[test]
fn test_visit_borrowed_str() {
    let visitor = Visitor;
    let input = "test_string";
    let result: Result<&&str, TestError> = visitor.visit_borrowed_str(input);
    assert_eq!(result.unwrap(), &input);
}

#[test]
#[should_panic]
fn test_visit_borrowed_str_invalid() {
    let visitor = Visitor;
    let result: Result<&&str, TestError> = visitor.visit_borrowed_str("");
    assert!(result.is_ok());
}


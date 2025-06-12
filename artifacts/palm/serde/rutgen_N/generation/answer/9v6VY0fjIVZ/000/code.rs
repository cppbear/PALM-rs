// Answer 0

#[derive(Debug)]
struct Content {
    value: f64,
}

impl Content {
    fn F64(value: f64) -> Self {
        Content { value }
    }
}

mod de {
    pub trait Error {}
}

#[derive(Debug)]
struct SimpleError;

impl de::Error for SimpleError {}

struct Visitor;

impl Visitor {
    fn visit_f64<F>(self, value: f64) -> Result<Content, F>
    where
        F: de::Error,
    {
        Ok(Content::F64(value))
    }
}

#[test]
fn test_visit_f64() {
    let visitor = Visitor;
    let result: Result<Content, SimpleError> = visitor.visit_f64(3.14);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value, 3.14);
}

#[test]
fn test_visit_f64_negative() {
    let visitor = Visitor;
    let result: Result<Content, SimpleError> = visitor.visit_f64(-1.0);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value, -1.0);
}

#[should_panic]
#[test]
fn test_visit_f64_should_panic() {
    let visitor = Visitor;
    let _result: Result<Content, SimpleError> = visitor.visit_f64(f64::NAN);
}


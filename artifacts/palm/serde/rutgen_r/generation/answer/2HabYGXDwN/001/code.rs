// Answer 0

#[derive(Debug)]
struct Content {
    value: u64,
}

impl Content {
    fn U64(value: u64) -> Self {
        Content { value }
    }
}

mod de {
    pub trait Error {}
}

struct MyError;

impl de::Error for MyError {}

struct MyVisitor;

impl MyVisitor {
    fn visit_u64<F>(self, value: u64) -> Result<Content, F>
    where
        F: de::Error,
    {
        Ok(Content::U64(value))
    }
}

#[test]
fn test_visit_u64() {
    let visitor = MyVisitor;

    let result = visitor.visit_u64::<MyError>(42);
    assert_eq!(result, Ok(Content::U64(42)));

    let result = visitor.visit_u64::<MyError>(0);
    assert_eq!(result, Ok(Content::U64(0)));

    let result = visitor.visit_u64::<MyError>(u64::MAX);
    assert_eq!(result, Ok(Content::U64(u64::MAX)));
}


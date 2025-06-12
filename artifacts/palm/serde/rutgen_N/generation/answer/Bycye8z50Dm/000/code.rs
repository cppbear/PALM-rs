// Answer 0

#[derive(Debug)]
struct IgnoredAny;

struct Visitor;

impl Visitor {
    fn visit_unit<E>(self) -> Result<IgnoredAny, E> {
        Ok(IgnoredAny)
    }
}

#[test]
fn test_visit_unit() {
    let visitor = Visitor;
    let result: Result<IgnoredAny, ()> = visitor.visit_unit();
    assert!(result.is_ok());
    match result {
        Ok(_) => (),
        Err(_) => panic!("Expected Ok, but got an Err"),
    }
}


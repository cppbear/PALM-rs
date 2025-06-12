// Answer 0

#[derive(Debug)]
struct DummyError;

mod de {
    pub trait Error {}
    impl Error for super::DummyError {}
}

struct ContentVisitor;

impl ContentVisitor {
    fn new() -> Self {
        ContentVisitor
    }

    fn visit_string(self, value: String) -> Result<String, super::DummyError> {
        Ok(value)
    }
}

enum TagOrContent {
    Tag,
    Content(String),
}

struct Visitor {
    name: String,
}

impl Visitor {
    fn visit_string<F>(self, value: String) -> Result<TagOrContent, F>
    where
        F: de::Error,
    {
        if value == self.name {
            Ok(TagOrContent::Tag)
        } else {
            ContentVisitor::new()
                .visit_string(value)
                .map(TagOrContent::Content)
        }
    }
}

#[test]
fn test_visit_string_not_equal() {
    let visitor = Visitor {
        name: String::from("expected_name"),
    };
    let input_value = String::from("different_name");
    
    match visitor.visit_string::<DummyError>(input_value) {
        Ok(TagOrContent::Content(content)) => {
            assert_eq!(content, "different_name");
        },
        _ => panic!("Expected Content variant with the input value"),
    }
}

#[test]
fn test_visit_string_boundary() {
    let visitor = Visitor {
        name: String::from("boundary_name"),
    };
    let input_value = String::from("not_boundary_name");
    
    match visitor.visit_string::<DummyError>(input_value) {
        Ok(TagOrContent::Content(content)) => {
            assert_eq!(content, "not_boundary_name");
        },
        _ => panic!("Expected Content variant with the input value"),
    }
}


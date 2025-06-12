// Answer 0

#[derive(Default)]
struct TestDe {
    read: TestRead,
    scratch: String,
}

#[derive(Default)]
struct TestRead {
    input: String,
}

impl TestRead {
    fn parse_str(&self, scratch: &mut String) -> Result<Reference, &'static str> {
        if self.input.is_empty() {
            Err("Empty input")
        } else {
            *scratch = self.input.clone();
            Ok(Reference::Borrowed(scratch.as_str()))
        }
    }
}

enum Reference<'a> {
    Borrowed(&'a str),
    Copied(String),
}

struct TestVisitor<'de> {
    value: Option<&'de str>,
}

impl<'de> de::Visitor<'de> for TestVisitor<'de> {
    type Value = &'de str;

    fn visit_borrowed_str(self, value: &'de str) -> Result<Self::Value> {
        self.value = Some(value);
        Ok(value)
    }

    fn visit_str(self, value: String) -> Result<Self::Value> {
        self.value = Some(&value);
        Ok(&value)
    }
}

#[test]
fn test_deserialize_any_borrowed() {
    let mut de = TestDe {
        read: TestRead {
            input: "test_string".to_string(),
        },
        ..Default::default()
    };
    
    let mut visitor = TestVisitor { value: None };
    
    let result = de.deserialize_any(visitor);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "test_string");
}

#[test]
#[should_panic(expected = "Empty input")]
fn test_deserialize_any_empty_input() {
    let mut de = TestDe {
        read: TestRead {
            input: "".to_string(),
        },
        ..Default::default()
    };
    
    let mut visitor = TestVisitor { value: None };
    
    de.deserialize_any(visitor).unwrap();  // This should panic
}


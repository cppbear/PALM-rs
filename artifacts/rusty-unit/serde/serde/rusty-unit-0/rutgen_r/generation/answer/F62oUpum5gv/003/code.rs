// Answer 0

#[derive(Debug)]
struct Content<'de> {
    data: &'de str,
}

struct TestVisitor {
    called: bool,
}

impl<'de> Visitor<'de> for TestVisitor {
    type Value = &'de str;

    fn visit_seq<V>(&mut self, seq: &mut V) -> Result<Self::Value, V::Error>
    where
        V: SeqAccess<'de>,
    {
        self.called = true;
        let content = seq.next_element::<Content<'de>>()?.unwrap();
        Ok(content.data)
    }
}

#[test]
fn test_visit_content_seq() {
    let content = vec![
        Content { data: "test1" },
        Content { data: "test2" },
    ];
    
    let mut visitor = TestVisitor { called: false };
    let result: Result<&str, _> = visit_content_seq(content, &mut visitor);
    
    assert!(visitor.called);
    assert_eq!(result, Ok("test1"));
}

#[test]
fn test_visit_content_seq_empty() {
    let content: Vec<Content> = vec![];
    
    let mut visitor = TestVisitor { called: false };
    let result: Result<&str, _> = visit_content_seq(content, &mut visitor);
    
    assert!(!visitor.called);
    assert_eq!(result.is_err(), true);
}


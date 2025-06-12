// Answer 0

#[test]
fn test_visit_array_ref_valid_case() {
    struct TestVisitor {
        count: usize,
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = usize;

        fn visit_seq<V>(self, _: V) -> Result<Self::Value, Error>
        where
            V: SeqAccess<'de>,
        {
            Ok(self.count)
        }
    }

    let array = vec![
        Value::Number(Number::from(1)),
        Value::String(String::from("test")),
    ];
    
    let visitor = TestVisitor { count: array.len() };

    let _ = visit_array_ref(&array, visitor);
}

#[test]
fn test_visit_array_ref_with_edge_case() {
    struct EdgeTestVisitor {
        count: usize,
    }

    impl<'de> Visitor<'de> for EdgeTestVisitor {
        type Value = usize;

        fn visit_seq<V>(self, _: V) -> Result<Self::Value, Error>
        where
            V: SeqAccess<'de>,
        {
            Ok(self.count)
        }
    }

    let edge_array = vec![
        Value::Bool(true),
        Value::Null,
        Value::Number(Number::from(3.14)),
    ];
    
    let edge_visitor = EdgeTestVisitor { count: edge_array.len() };

    let _ = visit_array_ref(&edge_array, edge_visitor);
}

#[test]
fn test_visit_array_ref_multiple_elements() {
    struct MultiVisitor {
        count: usize,
    }

    impl<'de> Visitor<'de> for MultiVisitor {
        type Value = usize;

        fn visit_seq<V>(self, _: V) -> Result<Self::Value, Error>
        where
            V: SeqAccess<'de>,
        {
            Ok(self.count)
        }
    }

    let multi_array = vec![
        Value::String(String::from("one")),
        Value::String(String::from("two")),
        Value::String(String::from("three")),
        Value::Number(Number::from(2)),
    ];

    let multi_visitor = MultiVisitor { count: multi_array.len() };

    let _ = visit_array_ref(&multi_array, multi_visitor);
} 

#[test]
fn test_visit_array_ref_small_size() {
    struct SmallVisitor {
        count: usize,
    }

    impl<'de> Visitor<'de> for SmallVisitor {
        type Value = usize;

        fn visit_seq<V>(self, _: V) -> Result<Self::Value, Error>
        where
            V: SeqAccess<'de>,
        {
            Ok(self.count)
        }
    }

    let small_array = vec![Value::Bool(false)];
    
    let small_visitor = SmallVisitor { count: small_array.len() };

    let _ = visit_array_ref(&small_array, small_visitor);
}


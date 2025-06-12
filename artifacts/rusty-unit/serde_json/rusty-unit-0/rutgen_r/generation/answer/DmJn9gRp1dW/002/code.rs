// Answer 0

#[derive(Default)]
struct FakeVisitor {
    return_ok: bool,
}

impl<'de> Visitor<'de> for FakeVisitor {
    type Value = String;

    fn visit_seq<M>(self, _seq: &mut M) -> Result<Self::Value, Error>
    where
        M: serde::de::SeqAccess<'de>,
    {
        if self.return_ok {
            Ok("valid".to_string())
        } else {
            Err(serde::de::Error::custom("Visit seq failed"))
        }
    }
}

#[test]
fn test_visit_array_with_ok_response() {
    let array = vec![Value::String("a".to_string()), Value::String("b".to_string())];
    let visitor = FakeVisitor { return_ok: true };

    let result = visit_array(array, visitor);
    assert!(result.is_ok());
}

#[test]
fn test_visit_array_with_partial_response() {
    let array = vec![Value::String("a".to_string()), Value::String("b".to_string())];
    let visitor = FakeVisitor { return_ok: false };

    let result = visit_array(array, visitor);
    assert!(result.is_err());
}

#[test]
fn test_visit_array_with_no_elements() {
    let array: Vec<Value> = Vec::new();
    let visitor = FakeVisitor { return_ok: true };

    let result = visit_array(array, visitor);
    assert!(result.is_ok());
}


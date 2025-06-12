// Answer 0

#[test]
fn test_visit_array_ref_with_err_from_visitor() {
    use serde_json::Value;
    use serde::de::{self, Visitor, SeqAccess};
    use serde::Deserialize;

    struct ErrVisitor;

    impl<'de> Visitor<'de> for ErrVisitor {
        type Value = ();

        fn visit_seq<V>(self, _seq: &mut V) -> Result<Self::Value, V::Error>
        where
            V: SeqAccess<'de>,
        {
            Err(de::Error::custom("intentional error"))
        }
    }

    let array: &[Value] = &[Value::Null, Value::Bool(true)];
    let visitor = ErrVisitor;

    let result = visit_array_ref(array, visitor);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().to_string(), "intentional error");
}

#[test]
fn test_visit_array_ref_with_fewer_elements() {
    use serde_json::Value;
    use serde::de::{self, Visitor, SeqAccess};
    use serde::Deserialize;

    struct PartialVisitor;

    impl<'de> Visitor<'de> for PartialVisitor {
        type Value = ();

        fn visit_seq<V>(self, seq: &mut V) -> Result<Self::Value, V::Error>
        where
            V: SeqAccess<'de>,
        {
            let _first: Value = seq.next_element()?.ok_or_else(|| de::Error::custom("no more elements"))?;
            Ok(())
        }
    }

    let array: &[Value] = &[Value::Null];
    let visitor = PartialVisitor;

    let result = visit_array_ref(array, visitor);
    assert!(result.is_err());
    assert_eq!(result.err().unwrap().to_string(), "invalid length 1: fewer elements in array");
}


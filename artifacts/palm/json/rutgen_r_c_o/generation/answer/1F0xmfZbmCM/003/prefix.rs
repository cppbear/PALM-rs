// Answer 0

#[test]
fn test_visit_array_ref_with_two_elements() {
    let array = [
        Value::Number(Number::from_str("1").unwrap()),
        Value::Number(Number::from_str("2").unwrap()),
    ];
    struct ValidVisitor;
    impl<'de> Visitor<'de> for ValidVisitor {
        type Value = Vec<i32>;

        fn visit_seq<V>(self, _seq: V) -> Result<Self::Value, Error>
        where
            V: SeqAccess<'de>,
        {
            Ok(vec![1, 2])
        }
    }
    let visitor = ValidVisitor;
    let result = visit_array_ref(&array, visitor);
}

#[test]
fn test_visit_array_ref_with_three_elements() {
    let array = [
        Value::Number(Number::from_str("1").unwrap()),
        Value::Number(Number::from_str("2").unwrap()),
        Value::Number(Number::from_str("3").unwrap()),
    ];
    struct ValidVisitor;
    impl<'de> Visitor<'de> for ValidVisitor {
        type Value = Vec<i32>;

        fn visit_seq<V>(self, _seq: V) -> Result<Self::Value, Error>
        where
            V: SeqAccess<'de>,
        {
            Ok(vec![1, 2, 3])
        }
    }
    let visitor = ValidVisitor;
    let result = visit_array_ref(&array, visitor);
}


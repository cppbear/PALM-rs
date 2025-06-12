// Answer 0

#[test]
fn test_deserialize_tuple_empty() {
    struct TestVisitor {
        result: Vec<i32>
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an integer sequence")
        }

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, Error>
        where
            V: SeqAccess<'de>,
        {
            while let Some(value) = seq.next_element()? {
                self.result.push(value);
            }
            Ok(self.result)
        }
    }

    let tuple_value = Value::Array(vec![]);
    let visitor = TestVisitor { result: Vec::new() };
    let result: Result<Vec<i32>, Error> = tuple_value.deserialize_tuple(0, visitor);
    assert_eq!(result.unwrap(), Vec::<i32>::new());
}

#[test]
fn test_deserialize_tuple_one_element() {
    struct TestVisitor {
        result: Vec<i32>
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an integer sequence")
        }

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, Error>
        where
            V: SeqAccess<'de>,
        {
            if let Some(value) = seq.next_element()? {
                self.result.push(value);
            }
            Ok(self.result)
        }
    }

    let tuple_value = Value::Array(vec![Value::Number(Number { n: 1 })]);
    let visitor = TestVisitor { result: Vec::new() };
    let result: Result<Vec<i32>, Error> = tuple_value.deserialize_tuple(1, visitor);
    assert_eq!(result.unwrap(), vec![1]);
}

#[test]
fn test_deserialize_tuple_multiple_elements() {
    struct TestVisitor {
        result: Vec<i32>
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an integer sequence")
        }

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, Error>
        where
            V: SeqAccess<'de>,
        {
            while let Some(value) = seq.next_element()? {
                self.result.push(value);
            }
            Ok(self.result)
        }
    }

    let tuple_value = Value::Array(vec![Value::Number(Number { n: 1 }), Value::Number(Number { n: 2 })]);
    let visitor = TestVisitor { result: Vec::new() };
    let result: Result<Vec<i32>, Error> = tuple_value.deserialize_tuple(2, visitor);
    assert_eq!(result.unwrap(), vec![1, 2]);
}

#[test]
#[should_panic]
fn test_deserialize_tuple_exceeding_length() {
    struct TestVisitor {
        result: Vec<i32>
    }

    impl<'de> Visitor<'de> for TestVisitor {
        type Value = Vec<i32>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an integer sequence")
        }

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, Error>
        where
            V: SeqAccess<'de>,
        {
            while let Some(value) = seq.next_element()? {
                self.result.push(value);
            }
            Ok(self.result)
        }
    }

    let tuple_value = Value::Array(vec![Value::Number(Number { n: 1 })]);
    let visitor = TestVisitor { result: Vec::new() };
    let _: Result<Vec<i32>, Error> = tuple_value.deserialize_tuple(2, visitor);
}


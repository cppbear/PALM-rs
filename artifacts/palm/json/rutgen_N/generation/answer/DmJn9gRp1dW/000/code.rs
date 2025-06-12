// Answer 0

#[derive(Debug)]
struct DummyVisitor {
    count: usize,
}

impl<'de> Visitor<'de> for DummyVisitor {
    type Value = usize;

    fn visit_seq<V>(self, seq: &mut V) -> Result<Self::Value, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let mut sum = 0;
        while let Some(_) = seq.next_element::<usize>()? {
            sum += 1;
        }
        Ok(sum)
    }
}

#[test]
fn test_visit_array_empty() {
    let array: Vec<Value> = vec![];
    let visitor = DummyVisitor { count: 0 };

    let result = visit_array(array, visitor);
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn test_visit_array_non_empty() {
    let array: Vec<Value> = vec![Value::U64(1), Value::U64(2)];
    let visitor = DummyVisitor { count: 0 };

    let result = visit_array(array, visitor);
    assert_eq!(result.unwrap(), 2);
}

#[test]
#[should_panic]
fn test_visit_array_too_few_elements() {
    let array: Vec<Value> = vec![Value::U64(1)];
    let visitor = DummyVisitor { count: 0 };

    let _result = visit_array(array, visitor);
}


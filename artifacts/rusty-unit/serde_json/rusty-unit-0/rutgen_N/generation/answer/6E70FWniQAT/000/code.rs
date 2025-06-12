// Answer 0

#[derive(Debug)]
struct Value;

struct SeqRefDeserializer<'de> {
    iter: std::slice::Iter<'de, Value>,
}

impl<'de> SeqRefDeserializer<'de> {
    fn new(slice: &'de [Value]) -> Self {
        SeqRefDeserializer { iter: slice.iter() }
    }
}

#[test]
fn test_new_with_empty_slice() {
    let values: &[Value] = &[];
    let deserializer = SeqRefDeserializer::new(values);
    assert!(deserializer.iter.len() == 0);
}

#[test]
fn test_new_with_single_value() {
    let value = Value;
    let values: &[Value] = &[value];
    let deserializer = SeqRefDeserializer::new(values);
    assert!(deserializer.iter.len() == 1);
}

#[test]
fn test_new_with_multiple_values() {
    let value1 = Value;
    let value2 = Value;
    let values: &[Value] = &[value1, value2];
    let deserializer = SeqRefDeserializer::new(values);
    assert!(deserializer.iter.len() == 2);
}


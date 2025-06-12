// Answer 0

#[derive(Debug)]
struct SeqDeserializer {
    iter: std::vec::IntoIter<Value>,
}

#[derive(Debug)]
struct Value;

impl SeqDeserializer {
    fn new(vec: Vec<Value>) -> Self {
        SeqDeserializer {
            iter: vec.into_iter(),
        }
    }
}

#[test]
fn test_empty_seq_deserializer() {
    let deserializer = SeqDeserializer::new(vec![]);
    assert!(deserializer.iter.next().is_none());
}

#[test]
fn test_single_value_seq_deserializer() {
    let value = Value;
    let deserializer = SeqDeserializer::new(vec![value]);

    assert!(deserializer.iter.next().is_some());
}

#[test]
fn test_multiple_values_seq_deserializer() {
    let value1 = Value;
    let value2 = Value;
    let deserializer = SeqDeserializer::new(vec![value1, value2]);

    assert!(deserializer.iter.next().is_some());
    assert!(deserializer.iter.next().is_some());
    assert!(deserializer.iter.next().is_none());
}


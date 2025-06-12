// Answer 0

#[derive(Debug)]
struct StringDeserializer {
    value: String,
    marker: std::marker::PhantomData<()>,
}

impl StringDeserializer {
    pub fn new(value: String) -> Self {
        StringDeserializer {
            value,
            marker: std::marker::PhantomData,
        }
    }
}

#[test]
fn test_new_with_non_empty_string() {
    let value = "test".to_string();
    let deserializer = StringDeserializer::new(value.clone());
    assert_eq!(deserializer.value, value);
}

#[test]
fn test_new_with_empty_string() {
    let value = "".to_string();
    let deserializer = StringDeserializer::new(value.clone());
    assert_eq!(deserializer.value, value);
}

#[test]
fn test_new_with_whitespace_string() {
    let value = "   ".to_string();
    let deserializer = StringDeserializer::new(value.clone());
    assert_eq!(deserializer.value, value);
}


// Answer 0

#[derive(Debug)]
struct StrDeserializer<'a> {
    value: &'a str,
    marker: std::marker::PhantomData<()>,
}

impl<'a> StrDeserializer<'a> {
    pub fn new(value: &'a str) -> Self {
        StrDeserializer {
            value,
            marker: std::marker::PhantomData,
        }
    }
}

#[test]
fn test_str_deserializer_new_valid_string() {
    let input = "test string";
    let deserializer = StrDeserializer::new(input);
    assert_eq!(deserializer.value, input);
}

#[test]
fn test_str_deserializer_new_empty_string() {
    let input = "";
    let deserializer = StrDeserializer::new(input);
    assert_eq!(deserializer.value, input);
}


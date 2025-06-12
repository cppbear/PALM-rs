// Answer 0

#[test]
fn test_cow_str_deserializer_empty_borrowed() {
    let deserializer = CowStrDeserializer::new(Cow::Borrowed(""));
}

#[test]
fn test_cow_str_deserializer_owned_test() {
    let deserializer = CowStrDeserializer::new(Cow::Owned(String::from("test")));
}

#[test]
fn test_cow_str_deserializer_owned_space() {
    let deserializer = CowStrDeserializer::new(Cow::Owned(String::from(" ")));
}

#[test]
fn test_cow_str_deserializer_borrowed_string() {
    let deserializer = CowStrDeserializer::new(Cow::Borrowed("test string"));
}


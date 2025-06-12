// Answer 0

#[test]
fn test_cow_str_deserializer_new() {
    use std::borrow::Cow;

    // Initialize the CowStrDeserializer with a borrowed string
    let borrowed_string: Cow<str> = Cow::Borrowed("test");
    let deserializer_borrowed = CowStrDeserializer::new(borrowed_string);

    assert_eq!(deserializer_borrowed.value, Cow::Borrowed("test"));

    // Initialize the CowStrDeserializer with an owned string
    let owned_string: Cow<str> = Cow::Owned(String::from("owned test"));
    let deserializer_owned = CowStrDeserializer::new(owned_string);

    assert_eq!(deserializer_owned.value, Cow::Owned(String::from("owned test")));
}


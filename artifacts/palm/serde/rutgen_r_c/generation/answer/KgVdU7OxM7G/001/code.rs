// Answer 0

#[test]
fn test_cow_str_deserializer_new() {
    use std::borrow::Cow;
    
    // Test with a borrowed string
    let borrowed: Cow<str> = Cow::Borrowed("Hello, borrowed!");
    let deserializer_borrowed = CowStrDeserializer::new(borrowed);
    assert_eq!(deserializer_borrowed.value, Cow::Borrowed("Hello, borrowed!"));

    // Test with an owned string
    let owned: Cow<str> = Cow::Owned(String::from("Hello, owned!"));
    let deserializer_owned = CowStrDeserializer::new(owned);
    assert_eq!(deserializer_owned.value, Cow::Owned("Hello, owned!".to_string()));

    // Test with an empty owned string
    let empty_owned: Cow<str> = Cow::Owned(String::from(""));
    let deserializer_empty_owned = CowStrDeserializer::new(empty_owned);
    assert_eq!(deserializer_empty_owned.value, Cow::Owned("".to_string()));
    
    // Test with an empty borrowed string
    let empty_borrowed: Cow<str> = Cow::Borrowed("");
    let deserializer_empty_borrowed = CowStrDeserializer::new(empty_borrowed);
    assert_eq!(deserializer_empty_borrowed.value, Cow::Borrowed(""));
}


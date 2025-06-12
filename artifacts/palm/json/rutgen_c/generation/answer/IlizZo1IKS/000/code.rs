// Answer 0

#[test]
fn test_borrowed_cow_str_deserializer_new() {
    use alloc::borrow::Cow;

    let input: Cow<str> = Cow::Borrowed("Test string");
    let deserializer = BorrowedCowStrDeserializer::new(input.clone());
    
    assert_eq!(deserializer.value, input);
}

#[test]
fn test_borrowed_cow_str_deserializer_new_owned() {
    use alloc::borrow::Cow;

    let input: Cow<str> = Cow::Owned(String::from("Owned test string"));
    let deserializer = BorrowedCowStrDeserializer::new(input.clone());

    assert_eq!(deserializer.value, input);
}


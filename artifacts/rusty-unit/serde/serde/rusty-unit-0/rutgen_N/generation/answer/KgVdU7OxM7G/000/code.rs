// Answer 0

#[test]
fn test_new_cowstr_deserializer() {
    use std::borrow::Cow;
    use std::marker::PhantomData;

    struct CowStrDeserializer<'a> {
        value: Cow<'a, str>,
        marker: PhantomData<&'a ()>,
    }

    let owned_str = String::from("test string");
    let cow_owned: Cow<str> = Cow::Owned(owned_str);
    let cow_ref: Cow<str> = Cow::Borrowed("borrowed string");

    let deserializer_owned = CowStrDeserializer::new(cow_owned);
    let deserializer_borrowed = CowStrDeserializer::new(cow_ref);

    assert_eq!(deserializer_owned.value, "test string");
    assert_eq!(deserializer_borrowed.value, "borrowed string");
}


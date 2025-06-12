// Answer 0

#[test]
fn test_new_with_borrowed_string() {
    use std::borrow::Cow;

    struct BorrowedCowStrDeserializer {
        value: Cow<'static, str>,
    }

    let borrowed_string: Cow<'static, str> = Cow::Borrowed("test string");
    let deserializer = BorrowedCowStrDeserializer { value: borrowed_string };
    assert_eq!(deserializer.value, Cow::Borrowed("test string"));
}

#[test]
fn test_new_with_owned_string() {
    use std::borrow::Cow;

    struct BorrowedCowStrDeserializer {
        value: Cow<'static, str>,
    }

    let owned_string: Cow<'static, str> = Cow::Owned(String::from("owned string"));
    let deserializer = BorrowedCowStrDeserializer { value: owned_string };
    assert_eq!(deserializer.value, Cow::Owned(String::from("owned string")));
}

#[test]
#[should_panic]
fn test_new_with_empty_string() {
    use std::borrow::Cow;

    struct BorrowedCowStrDeserializer {
        value: Cow<'static, str>,
    }

    let empty_string: Cow<'static, str> = Cow::Borrowed("");
    let deserializer = BorrowedCowStrDeserializer { value: empty_string };
    assert!(deserializer.value.is_empty());
}


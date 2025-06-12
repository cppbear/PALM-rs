// Answer 0

#[test]
fn test_string_deserializer_new() {
    use std::marker::PhantomData;

    struct ErrorImpl;

    struct TestStringDeserializer {
        value: String,
        marker: PhantomData<ErrorImpl>,
    }

    impl TestStringDeserializer {
        pub fn new(value: String) -> Self {
            TestStringDeserializer {
                value,
                marker: PhantomData,
            }
        }
    }

    let value = String::from("test");
    let deserializer = TestStringDeserializer::new(value.clone());

    assert_eq!(deserializer.value, value);
}

#[test]
fn test_string_deserializer_new_empty() {
    use std::marker::PhantomData;

    struct ErrorImpl;

    struct TestStringDeserializer {
        value: String,
        marker: PhantomData<ErrorImpl>,
    }

    impl TestStringDeserializer {
        pub fn new(value: String) -> Self {
            TestStringDeserializer {
                value,
                marker: PhantomData,
            }
        }
    }

    let value = String::from("");
    let deserializer = TestStringDeserializer::new(value.clone());

    assert_eq!(deserializer.value, value);
}


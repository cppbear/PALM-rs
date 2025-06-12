// Answer 0

#[test]
fn test_str_deserializer_new_with_valid_string() {
    struct StrDeserializer<'a> {
        value: &'a str,
        marker: std::marker::PhantomData<&'a ()>,
    }

    impl<'a> StrDeserializer<'a> {
        pub fn new(value: &'a str) -> Self {
            StrDeserializer {
                value,
                marker: std::marker::PhantomData,
            }
        }
    }

    let input = "test string";
    let deserializer = StrDeserializer::new(input);
    assert_eq!(deserializer.value, input);
}

#[test]
fn test_str_deserializer_new_with_empty_string() {
    struct StrDeserializer<'a> {
        value: &'a str,
        marker: std::marker::PhantomData<&'a ()>,
    }

    impl<'a> StrDeserializer<'a> {
        pub fn new(value: &'a str) -> Self {
            StrDeserializer {
                value,
                marker: std::marker::PhantomData,
            }
        }
    }

    let input = "";
    let deserializer = StrDeserializer::new(input);
    assert_eq!(deserializer.value, input);
}

#[should_panic]
#[test]
fn test_str_deserializer_new_with_null_string() {
    struct StrDeserializer<'a> {
        value: &'a str,
        marker: std::marker::PhantomData<&'a ()>,
    }

    impl<'a> StrDeserializer<'a> {
        pub fn new(value: &'a str) -> Self {
            StrDeserializer {
                value,
                marker: std::marker::PhantomData,
            }
        }
    }

    let input: &'static str = std::ptr::null::<str>() as *const _ as &'static str;
    StrDeserializer::new(input);
}


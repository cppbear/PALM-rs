// Answer 0

#[test]
fn test_into_deserializer_basic() {
    struct Deserializer;

    impl Deserializer {
        fn into_deserializer(self) -> Self {
            self
        }
    }

    let deserializer = Deserializer;
    let result = deserializer.into_deserializer();

    assert_eq!(std::ptr::eq(&deserializer, &result), true);
}

#[test]
fn test_into_deserializer_multiple_calls() {
    struct Deserializer;

    impl Deserializer {
        fn into_deserializer(self) -> Self {
            self
        }
    }

    let deserializer = Deserializer;
    let result1 = deserializer.into_deserializer();
    let result2 = result1.into_deserializer();

    assert_eq!(std::ptr::eq(&result1, &result2), true);
}

#[test]
fn test_into_deserializer_boundary_case() {
    struct Deserializer;

    impl Deserializer {
        fn into_deserializer(self) -> Self {
            self
        }
    }

    let deserializer = Deserializer;
    let result = deserializer.into_deserializer();

    // Confirm that the result is indeed the same deserializer
    assert!(std::ptr::eq(&deserializer, &result));
}


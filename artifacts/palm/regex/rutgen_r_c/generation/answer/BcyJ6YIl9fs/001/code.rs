// Answer 0

#[test]
fn test_is_always_utf8_class_bytes_all_ascii() {
    struct MockClassBytesRange;
    struct MockClassBytes {
        set: IntervalSet<MockClassBytesRange>,
    }

    impl MockClassBytes {
        fn new() -> Self {
            Self { set: IntervalSet::new() }
        }

        fn is_all_ascii(&self) -> bool {
            true // Mocked to return true for all ASCII
        }
    }

    let class_bytes = Class::Bytes(MockClassBytes::new());
    assert!(class_bytes.is_always_utf8());
}

#[test]
fn test_is_always_utf8_class_bytes_non_ascii() {
    struct MockClassBytesRange;
    struct MockClassBytes {
        set: IntervalSet<MockClassBytesRange>,
    }

    impl MockClassBytes {
        fn new() -> Self {
            Self { set: IntervalSet::new() }
        }

        fn is_all_ascii(&self) -> bool {
            false // Mocked to return false indicating non-ASCII presence
        }
    }

    let class_bytes = Class::Bytes(MockClassBytes::new());
    assert!(!class_bytes.is_always_utf8());
}


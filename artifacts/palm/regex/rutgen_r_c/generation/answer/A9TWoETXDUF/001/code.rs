// Answer 0

#[test]
fn test_canonical_value_valid_input() {
    struct DummyPropertyValues;

    impl PropertyValues for DummyPropertyValues {
        fn as_slice(&self) -> &[&str] {
            &["value1", "value2", "value3"]
        }
    }

    let vals = DummyPropertyValues;
    let result = canonical_value(vals, "value1");
    assert_eq!(result, Some("value1"));
}

#[test]
fn test_canonical_value_partial_match() {
    struct DummyPropertyValues;

    impl PropertyValues for DummyPropertyValues {
        fn as_slice(&self) -> &[&str] {
            &["valid", "property"]
        }
    }

    let vals = DummyPropertyValues;
    let result = canonical_value(vals, "invalid");
    assert_eq!(result, None);
}

#[test]
fn test_canonical_value_empty_string() {
    struct DummyPropertyValues;

    impl PropertyValues for DummyPropertyValues {
        fn as_slice(&self) -> &[&str] {
            &["someValue"]
        }
    }

    let vals = DummyPropertyValues;
    let result = canonical_value(vals, "");
    assert_eq!(result, None);
}

#[test]
fn test_canonical_value_nonexistent_property() {
    struct DummyPropertyValues;

    impl PropertyValues for DummyPropertyValues {
        fn as_slice(&self) -> &[&str] {
            &[]
        }
    }

    let vals = DummyPropertyValues;
    let result = canonical_value(vals, "unknownValue");
    assert_eq!(result, None);
}


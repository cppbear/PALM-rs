// Answer 0

#[test]
fn test_fmt_with_valid_entry() {
    struct IndexedEntry {
        index: usize,
        key: String,
        value: String,
    }

    impl IndexedEntry {
        fn key(&self) -> &String {
            &self.key
        }

        fn get(&self) -> &String {
            &self.value
        }
    }

    let entry = IndexedEntry {
        index: 1,
        key: String::from("key1"),
        value: String::from("value1"),
    };

    let mut output = String::new();
    let result = write!(&mut output, "{:?}", entry);
    assert!(result.is_ok());
    assert!(output.contains("IndexedEntry"));
    assert!(output.contains("index: 1"));
    assert!(output.contains("key: \"key1\""));
    assert!(output.contains("value: \"value1\""));
}

#[test]
fn test_fmt_with_empty_key_value() {
    struct IndexedEntry {
        index: usize,
        key: String,
        value: String,
    }

    impl IndexedEntry {
        fn key(&self) -> &String {
            &self.key
        }

        fn get(&self) -> &String {
            &self.value
        }
    }

    let entry = IndexedEntry {
        index: 2,
        key: String::from(""),
        value: String::from(""),
    };

    let mut output = String::new();
    let result = write!(&mut output, "{:?}", entry);
    assert!(result.is_ok());
    assert!(output.contains("IndexedEntry"));
    assert!(output.contains("index: 2"));
    assert!(output.contains("key: \"\""));
    assert!(output.contains("value: \"\""));
}

#[test]
fn test_fmt_with_special_characters() {
    struct IndexedEntry {
        index: usize,
        key: String,
        value: String,
    }

    impl IndexedEntry {
        fn key(&self) -> &String {
            &self.key
        }

        fn get(&self) -> &String {
            &self.value
        }
    }

    let entry = IndexedEntry {
        index: 3,
        key: String::from("key_with_special_$%@!"),
        value: String::from("value_with_special_<>@#"),
    };

    let mut output = String::new();
    let result = write!(&mut output, "{:?}", entry);
    assert!(result.is_ok());
    assert!(output.contains("IndexedEntry"));
    assert!(output.contains("index: 3"));
    assert!(output.contains("key: \"key_with_special_$%@!\""));
    assert!(output.contains("value: \"value_with_special_<>@#\""));
}


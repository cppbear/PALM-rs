// Answer 0

#[test]
fn test_fmt_occupied_entry() {
    use std::fmt;
    
    struct OccupiedEntry<'a, K: fmt::Debug, V: fmt::Debug> {
        key: &'a K,
        value: &'a V,
    }

    impl<'a, K: fmt::Debug, V: fmt::Debug> OccupiedEntry<'a, K, V> {
        fn key(&self) -> &K {
            self.key
        }

        fn get(&self) -> &V {
            self.value
        }

        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("OccupiedEntry")
                .field("key", self.key())
                .field("value", self.get())
                .finish()
        }
    }

    let key = String::from("example_key");
    let value = String::from("example_value");
    
    let entry = OccupiedEntry {
        key: &key,
        value: &value,
    };

    let mut output = Vec::new();
    {
        let f = &mut fmt::Formatter::new(&mut output);
        assert!(entry.fmt(f).is_ok());
    }

    let output_string = String::from_utf8(output).unwrap();
    assert!(output_string.contains("key: \"example_key\""));
    assert!(output_string.contains("value: \"example_value\""));
}

#[test]
#[should_panic]
fn test_fmt_occupied_entry_panic() {
    struct OccupiedEntry<'a, K: fmt::Debug, V: fmt::Debug> {
        key: Option<&'a K>,
        value: Option<&'a V>,
    }

    impl<'a, K: fmt::Debug, V: fmt::Debug> OccupiedEntry<'a, K, V> {
        fn key(&self) -> &K {
            self.key.expect("Key is missing")
        }

        fn get(&self) -> &V {
            self.value.expect("Value is missing")
        }

        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("OccupiedEntry")
                .field("key", self.key())
                .field("value", self.get())
                .finish()
        }
    }

    let entry = OccupiedEntry {
        key: None,
        value: None,
    };

    let _ = entry.fmt(&mut fmt::Formatter::new(&mut Vec::new()));
}


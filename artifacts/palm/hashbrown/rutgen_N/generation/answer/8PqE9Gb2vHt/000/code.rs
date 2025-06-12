// Answer 0

#[test]
fn test_fmt_occupied_entry() {
    use std::fmt;

    struct OccupiedEntry<'a, K, V> {
        key: &'a K,
        value: V,
    }

    impl<'a, K: fmt::Debug, V: fmt::Debug> OccupiedEntry<'a, K, V> {
        fn key(&self) -> &K {
            self.key
        }

        fn get(&self) -> &V {
            &self.value
        }

        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("OccupiedEntry")
                .field("key", self.key())
                .field("value", self.get())
                .finish()
        }
    }

    let key = "test_key";
    let value = 42;
    let entry = OccupiedEntry { key: &key, value };

    let mut output = String::new();
    {
        let mut formatter = fmt::Formatter::new(&mut output);
        entry.fmt(&mut formatter).unwrap();
    }

    assert!(output.contains("key: \"test_key\""));
    assert!(output.contains("value: 42"));
}


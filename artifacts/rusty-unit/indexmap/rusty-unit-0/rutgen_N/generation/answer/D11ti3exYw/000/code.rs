// Answer 0

#[test]
fn test_raw_occupied_entry_mut_fmt() {
    struct TestEntry {
        key: String,
        value: i32,
    }

    impl TestEntry {
        fn key(&self) -> &String {
            &self.key
        }

        fn get(&self) -> &i32 {
            &self.value
        }

        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("RawOccupiedEntryMut")
                .field("key", self.key())
                .field("value", self.get())
                .finish_non_exhaustive()
        }
    }

    let entry = TestEntry {
        key: "test_key".to_string(),
        value: 42,
    };
    
    let mut output = String::new();
    let mut formatter = std::fmt::Formatter::new(&mut output);
    
    entry.fmt(&mut formatter).unwrap();
    
    assert!(output.contains("key: test_key"));
    assert!(output.contains("value: 42"));
}


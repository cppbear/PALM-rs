// Answer 0

#[derive(Debug)]
struct OccupiedEntry<K, V> {
    key: K,
    value: V,
}

impl<K: std::fmt::Debug, V: std::fmt::Debug> OccupiedEntry<K, V> {
    fn key(&self) -> &K {
        &self.key
    }

    fn get(&self) -> &V {
        &self.value
    }
}

impl<K: std::fmt::Debug, V: std::fmt::Debug> std::fmt::Debug for OccupiedEntry<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OccupiedEntry")
            .field("key", self.key())
            .field("value", self.get())
            .finish()
    }
}

#[test]
fn test_fmt_occupied_entry() {
    let entry = OccupiedEntry {
        key: "test_key",
        value: 42,
    };
    
    let mut output = String::new();
    let result = writeln!(&mut output, "{:?}", entry);
    
    assert!(result.is_ok());
    assert!(output.contains("OccupiedEntry"));
    assert!(output.contains("key: \"test_key\""));
    assert!(output.contains("value: 42"));
}

#[test]
fn test_fmt_occupied_entry_empty() {
    let entry: OccupiedEntry<&str, &str> = OccupiedEntry { key: "", value: "" };
    
    let mut output = String::new();
    let result = writeln!(&mut output, "{:?}", entry);
    
    assert!(result.is_ok());
    assert!(output.contains("OccupiedEntry"));
    assert!(output.contains("key: \"\""));
    assert!(output.contains("value: \"\""));
}


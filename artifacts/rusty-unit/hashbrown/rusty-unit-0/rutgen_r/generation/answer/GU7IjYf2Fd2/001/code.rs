// Answer 0

#[derive(Debug)]
struct MockEntry<'a> {
    key: &'a str,
    old_value: Option<&'a str>,
}

impl<'a> MockEntry<'a> {
    fn key(&self) -> &str {
        self.key
    }
    
    fn get(&self) -> Option<&str> {
        self.old_value
    }
}

struct OccupiedError<'a> {
    entry: MockEntry<'a>,
    value: &'a str,
}

impl<'a> std::fmt::Debug for OccupiedError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OccupiedError")
            .field("key", self.entry.key())
            .field("old_value", self.entry.get())
            .field("new_value", &self.value)
            .finish()
    }
}

#[test]
fn test_occupied_error_with_no_old_value() {
    let entry = MockEntry {
        key: "test_key",
        old_value: None,
    };
    let error = OccupiedError {
        entry,
        value: "new_value",
    };
    let fmt_result = format!("{:?}", error);
    assert!(fmt_result.contains("test_key"));
    assert!(fmt_result.contains("old_value: None"));
    assert!(fmt_result.contains("new_value: new_value"));
}

#[test]
fn test_occupied_error_with_old_value() {
    let entry = MockEntry {
        key: "test_key",
        old_value: Some("old_value"),
    };
    let error = OccupiedError {
        entry,
        value: "new_value",
    };
    let fmt_result = format!("{:?}", error);
    assert!(fmt_result.contains("test_key"));
    assert!(fmt_result.contains("old_value: Some(\"old_value\")"));
    assert!(fmt_result.contains("new_value: new_value"));
}

#[test]
#[should_panic]
fn test_occupied_error_with_panic_triggering_case() {
    let entry = MockEntry {
        key: "panic_key",
        old_value: Some("should_panic"),
    };
    let error = OccupiedError {
        entry,
        value: "panic_trigger",
    };
    panic!("This will trigger a panic for test.");
    let _ = format!("{:?}", error);
}


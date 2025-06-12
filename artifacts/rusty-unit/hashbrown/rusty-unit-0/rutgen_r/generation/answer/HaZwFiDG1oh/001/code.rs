// Answer 0


use std::fmt;

struct OccupiedEntry<'a, T> {
    value: &'a T,
}

impl<'a, T> OccupiedEntry<'a, T> {
    fn get(&self) -> &T {
        self.value
    }
}

impl<'a, T: fmt::Debug> fmt::Debug for OccupiedEntry<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OccupiedEntry")
            .field("value", self.get())
            .finish()
    }
}

#[test]
fn test_occupied_entry_debug() {
    let value = 42;
    let entry = OccupiedEntry { value: &value };
    let result = format!("{:?}", entry);
    assert_eq!(result, "OccupiedEntry { value: 42 }");
}

#[test]
fn test_occupied_entry_debug_empty_value() {
    let value: Option<i32> = None;
    let entry = OccupiedEntry { value: &value };
    let result = format!("{:?}", entry);
    assert_eq!(result, "OccupiedEntry { value: None }");
}

#[should_panic]
#[test]
fn test_occupied_entry_panic_on_invalid_value() {
    struct InvalidValue;
    let entry = OccupiedEntry { value: &InvalidValue };
    let _ = format!("{:?}", entry);
}



// Answer 0

#[derive(Debug)]
struct MyStruct {
    entries: Vec<(String, i32)>,
}

impl MyStruct {
    fn new(entries: Vec<(String, i32)>) -> Self {
        MyStruct { entries }
    }

    fn into_entries(self) -> Vec<(String, i32)> {
        self.entries
    }
}

#[test]
fn test_into_entries_non_empty() {
    let my_struct = MyStruct::new(vec![("key1".to_string(), 1), ("key2".to_string(), 2)]);
    let entries = my_struct.into_entries();
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0], ("key1".to_string(), 1));
    assert_eq!(entries[1], ("key2".to_string(), 2));
}

#[test]
fn test_into_entries_empty() {
    let my_struct = MyStruct::new(vec![]);
    let entries: Vec<(String, i32)> = my_struct.into_entries();
    assert_eq!(entries.len(), 0);
}


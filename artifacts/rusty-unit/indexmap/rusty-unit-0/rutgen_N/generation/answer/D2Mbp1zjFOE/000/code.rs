// Answer 0

#[derive(Debug)]
struct MyStruct {
    map: MyMap,
}

#[derive(Debug)]
struct MyMap {
    entries: Vec<(String, i32)>,
}

impl MyMap {
    fn into_entries(self) -> Vec<(String, i32)> {
        self.entries
    }
}

impl MyStruct {
    fn into_entries(self) -> Vec<(String, i32)> {
        self.map.into_entries()
    }
}

#[test]
fn test_into_entries_non_empty() {
    let my_map = MyMap {
        entries: vec![
            ("key1".to_string(), 1),
            ("key2".to_string(), 2),
        ],
    };
    
    let my_struct = MyStruct { map: my_map };
    
    let entries = my_struct.into_entries();
    
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0], ("key1".to_string(), 1));
    assert_eq!(entries[1], ("key2".to_string(), 2));
}

#[test]
fn test_into_entries_empty() {
    let my_map = MyMap {
        entries: Vec::new(),
    };
    
    let my_struct = MyStruct { map: my_map };
    
    let entries = my_struct.into_entries();
    
    assert_eq!(entries.len(), 0);
}


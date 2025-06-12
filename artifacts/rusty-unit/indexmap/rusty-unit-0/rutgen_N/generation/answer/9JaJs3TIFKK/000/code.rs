// Answer 0

#[derive(Debug)]
struct MyEntry {
    key: i32,
    value: String,
}

struct MyMap {
    entries: Vec<MyEntry>,
}

impl MyMap {
    fn as_entries_mut(&mut self) -> &mut [MyEntry] {
        &mut self.entries
    }
}

struct MySet {
    map: MyMap,
}

impl MySet {
    fn as_entries_mut(&mut self) -> &mut [MyEntry] {
        self.map.as_entries_mut()
    }
}

#[test]
fn test_as_entries_mut() {
    let mut my_set = MySet {
        map: MyMap {
            entries: vec![
                MyEntry { key: 1, value: "one".to_string() },
                MyEntry { key: 2, value: "two".to_string() },
            ],
        },
    };
    
    let entries = my_set.as_entries_mut();
    
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].key, 1);
    assert_eq!(entries[1].key, 2);
    
    entries[0].value = "updated".to_string();
    assert_eq!(entries[0].value, "updated");
}

#[test]
fn test_as_entries_mut_empty() {
    let mut my_set = MySet {
        map: MyMap {
            entries: vec![],
        },
    };

    let entries = my_set.as_entries_mut();
    
    assert_eq!(entries.len(), 0);
}


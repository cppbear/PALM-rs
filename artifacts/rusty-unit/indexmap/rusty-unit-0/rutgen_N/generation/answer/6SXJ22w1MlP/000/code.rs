// Answer 0

#[derive(Debug)]
struct Core {
    entries: Vec<i32>,
}

#[derive(Debug)]
struct MyMap {
    core: Core,
}

impl MyMap {
    fn new() -> Self {
        Self {
            core: Core { entries: Vec::new() },
        }
    }

    fn as_entries_mut(&mut self) -> &mut [i32] {
        &mut self.core.entries
    }
}

#[test]
fn test_as_entries_mut_empty() {
    let mut map = MyMap::new();
    let entries = map.as_entries_mut();
    assert_eq!(entries.len(), 0);
}

#[test]
fn test_as_entries_mut_non_empty() {
    let mut map = MyMap::new();
    map.core.entries.push(1);
    map.core.entries.push(2);
    let entries = map.as_entries_mut();
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0], 1);
    assert_eq!(entries[1], 2);
}

#[test]
fn test_as_entries_mut_modify() {
    let mut map = MyMap::new();
    map.core.entries.push(1);
    let entries = map.as_entries_mut();
    entries[0] = 10;
    assert_eq!(map.core.entries[0], 10);
}


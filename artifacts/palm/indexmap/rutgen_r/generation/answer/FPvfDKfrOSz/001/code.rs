// Answer 0

#[derive(Debug)]
struct Core {
    entries: Vec<i32>,
}

impl Core {
    fn as_entries(&self) -> &[i32] {
        &self.entries
    }
}

struct Map {
    core: Core,
}

impl Map {
    fn new(entries: Vec<i32>) -> Self {
        Map {
            core: Core { entries },
        }
    }

    fn as_entries(&self) -> &[i32] {
        self.core.as_entries()
    }
}

#[test]
fn test_as_entries_non_empty() {
    let map = Map::new(vec![1, 2, 3]);
    let entries = map.as_entries();
    assert_eq!(entries, &[1, 2, 3]);
}

#[test]
fn test_as_entries_empty() {
    let map = Map::new(vec![]);
    let entries = map.as_entries();
    assert_eq!(entries, &[]);
}

#[should_panic]
#[test]
fn test_as_entries_panic_on_empty_core() {
    let core = Core { entries: Vec::new() };
    let map = Map { core };
    assert!(map.as_entries().is_empty()); // This should not panic, just an example
}


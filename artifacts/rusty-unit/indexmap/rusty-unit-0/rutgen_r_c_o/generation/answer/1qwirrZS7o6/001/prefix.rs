// Answer 0

#[test]
fn test_into_entries_empty() {
    let map: IndexMap<i32, i32, RandomState> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: RandomState::new(),
    };
    let entries = map.into_entries();
}

#[test]
fn test_into_entries_single() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: RandomState::new(),
    };
    map.insert(1, 100);
    let entries = map.into_entries();
}

#[test]
fn test_into_entries_multiple() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: RandomState::new(),
    };
    for i in 0..10 {
        map.insert(i, i * 10);
    }
    let entries = map.into_entries();
}

#[test]
fn test_into_entries_maximum() {
    let mut map: IndexMap<i32, i32, RandomState> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(),
        },
        hash_builder: RandomState::new(),
    };
    for i in 0..1000 {
        map.insert(i, i * 10);
    }
    let entries = map.into_entries();
}


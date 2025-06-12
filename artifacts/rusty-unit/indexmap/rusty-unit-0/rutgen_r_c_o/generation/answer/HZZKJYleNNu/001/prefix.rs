// Answer 0

#[test]
fn test_iter_mut2_with_non_empty_entries() {
    let mut index_map: IndexMap<i32, String, std::collections::hash_map::RandomState> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(vec![
                Bucket { hash: HashValue::new(1), key: 1, value: String::from("one") },
                Bucket { hash: HashValue::new(2), key: 2, value: String::from("two") },
            ]),
        },
        hash_builder: std::collections::hash_map::RandomState::new(),
    };
    let mut iter = index_map.iter_mut2();
}

#[test]
fn test_iter_mut2_with_single_entry() {
    let mut index_map: IndexMap<i32, String, std::collections::hash_map::RandomState> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(vec![Bucket { hash: HashValue::new(1), key: 1, value: String::from("one") }]),
        },
        hash_builder: std::collections::hash_map::RandomState::new(),
    };
    let mut iter = index_map.iter_mut2();
}

#[test]
fn test_iter_mut2_with_multiple_entries() {
    let mut index_map: IndexMap<i32, String, std::collections::hash_map::RandomState> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(vec![
                Bucket { hash: HashValue::new(1), key: 1, value: String::from("one") },
                Bucket { hash: HashValue::new(2), key: 2, value: String::from("two") },
                Bucket { hash: HashValue::new(3), key: 3, value: String::from("three") },
            ]),
        },
        hash_builder: std::collections::hash_map::RandomState::new(),
    };
    let mut iter = index_map.iter_mut2();
}

#[test]
#[should_panic]
fn test_iter_mut2_on_empty_entries() {
    let mut index_map: IndexMap<i32, String, std::collections::hash_map::RandomState> = IndexMap {
        core: IndexMapCore {
            indices: Indices::new(),
            entries: Entries::new(vec![]),
        },
        hash_builder: std::collections::hash_map::RandomState::new(),
    };
    let mut iter = index_map.iter_mut2();
}


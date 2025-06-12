// Answer 0

#[test]
fn test_insert_updates_value_and_returns_old() {
    struct TestMap {
        entries: Vec<(String, i32)>,
    }

    struct TestRefMut<'a> {
        entries: &'a mut TestMap,
    }

    impl<'a> TestRefMut<'a> {
        fn new(entries: &'a mut TestMap) -> Self {
            Self { entries }
        }
    }

    let mut test_map = TestMap {
        entries: vec![("key1".to_string(), 10)],
    };
    let mut ref_mut = TestRefMut::new(&mut test_map);
    let index = 0;

    let mut indexed_entry = IndexedEntry::new(&mut ref_mut, index);
    let old_value = indexed_entry.insert(20);

    assert_eq!(old_value, 10);
    assert_eq!(test_map.entries[index].1, 20);
}

#[test]
fn test_insert_overwrites_existing_value() {
    struct TestMap {
        entries: Vec<(String, i32)>,
    }

    struct TestRefMut<'a> {
        entries: &'a mut TestMap,
    }

    impl<'a> TestRefMut<'a> {
        fn new(entries: &'a mut TestMap) -> Self {
            Self { entries }
        }
    }

    let mut test_map = TestMap {
        entries: vec![("key2".to_string(), 30)],
    };
    let mut ref_mut = TestRefMut::new(&mut test_map);
    let index = 0;

    let mut indexed_entry = IndexedEntry::new(&mut ref_mut, index);
    let old_value = indexed_entry.insert(40);

    assert_eq!(old_value, 30);
    assert_eq!(test_map.entries[index].1, 40);
}


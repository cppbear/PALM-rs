// Answer 0

#[test]
fn test_key_mut() {
    struct DummyMap;
    struct DummyValue;

    struct DummyVacantEntry<'a> {
        map: &'a mut DummyMap,
        hash: usize,
        key: i32,
    }

    impl<'a> VacantEntry<'a, i32, DummyValue> {
        fn new(map: &'a mut DummyMap, hash: usize, key: i32) -> Self {
            Self { map, hash, key }
        }
    }

    let mut dummy_map = DummyMap;
    let mut entry = DummyVacantEntry::new(&mut dummy_map, 0, 100);

    let key_mut_ref = entry.key_mut();
    *key_mut_ref = 200;

    assert_eq!(entry.key, 200);
}

#[test]
fn test_key_mut_multiple_entries() {
    struct DummyMap;
    struct DummyValue;

    struct DummyVacantEntry<'a> {
        map: &'a mut DummyMap,
        hash: usize,
        key: i32,
    }

    impl<'a> VacantEntry<'a, i32, DummyValue> {
        fn new(map: &'a mut DummyMap, hash: usize, key: i32) -> Self {
            Self { map, hash, key }
        }
    }

    let mut dummy_map = DummyMap;
    let mut entry1 = DummyVacantEntry::new(&mut dummy_map, 0, 100);
    let mut entry2 = DummyVacantEntry::new(&mut dummy_map, 0, 150);

    {
        let key_mut_ref1 = entry1.key_mut();
        *key_mut_ref1 = 200;
    }

    {
        let key_mut_ref2 = entry2.key_mut();
        *key_mut_ref2 = 250;
    }

    assert_eq!(entry1.key, 200);
    assert_eq!(entry2.key, 250);
}


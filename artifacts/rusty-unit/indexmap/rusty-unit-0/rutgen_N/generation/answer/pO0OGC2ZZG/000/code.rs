// Answer 0

#[test]
fn test_as_entries() {
    struct TestStruct {
        map: TestMap,
    }

    struct TestMap {
        entries: Vec<TestEntry>,
    }

    struct TestEntry {
        key: i32,
        value: String,
    }

    impl TestMap {
        fn as_entries(&self) -> &[TestEntry] {
            &self.entries
        }
    }

    let entry1 = TestEntry { key: 1, value: "one".to_string() };
    let entry2 = TestEntry { key: 2, value: "two".to_string() };
    let test_map = TestMap {
        entries: vec![entry1, entry2],
    };
    let test_struct = TestStruct { map: test_map };

    let entries = test_struct.map.as_entries();
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].key, 1);
    assert_eq!(entries[0].value, "one");
    assert_eq!(entries[1].key, 2);
    assert_eq!(entries[1].value, "two");
}


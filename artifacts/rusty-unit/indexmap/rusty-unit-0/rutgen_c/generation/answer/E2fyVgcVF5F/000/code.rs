// Answer 0

#[test]
fn test_indexed_entry_get() {
    #[derive(Debug)]
    struct TestKey(u32);
    #[derive(Debug)]
    struct TestValue(u32);

    let mut entries = vec![
        Bucket {
            hash: HashValue::default(),
            key: TestKey(1),
            value: TestValue(10),
        },
        Bucket {
            hash: HashValue::default(),
            key: TestKey(2),
            value: TestValue(20),
        },
    ];

    let mut indices = // Initialize indices as appropriate for testing

    let mut map = IndexMapCore {
        entries,
        // Initialize other necessary fields
    };

    let entry = IndexedEntry::new(&mut map, 1);

    assert_eq!(entry.get(), &TestValue(20));
}

#[test]
fn test_indexed_entry_get_invalid_index() {
    #[derive(Debug)]
    struct TestKey(u32);
    #[derive(Debug)]
    struct TestValue(u32);

    let mut entries = vec![
        Bucket {
            hash: HashValue::default(),
            key: TestKey(1),
            value: TestValue(10),
        },
    ];

    let mut indices = // Initialize indices as appropriate for testing

    let mut map = IndexMapCore {
        entries,
        // Initialize other necessary fields
    };

    let entry = IndexedEntry::new(&mut map, 0);

    assert_eq!(entry.get(), &TestValue(10));
}


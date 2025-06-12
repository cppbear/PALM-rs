// Answer 0

#[test]
fn test_clear() {
    #[derive(Debug)]
    struct TestEquivalent;

    impl Equivalent<usize> for TestEquivalent {
        fn equivalent(&self, _: &usize) -> bool {
            true
        }
    }

    let mut index_map: IndexMapCore<TestEquivalent, String> = IndexMapCore::new();
    
    // Add some entries to the IndexMapCore
    index_map.entries.push(Bucket {
        hash: HashValue(1),
        key: TestEquivalent,
        value: String::from("value1"),
    });

    index_map.entries.push(Bucket {
        hash: HashValue(2),
        key: TestEquivalent,
        value: String::from("value2"),
    });

    assert_eq!(index_map.entries.len(), 2);
    
    // Clear the IndexMapCore
    index_map.clear();

    // Check that entries and indices are cleared
    assert!(index_map.entries.is_empty());
    assert_eq!(index_map.indices, Indices::new());
}


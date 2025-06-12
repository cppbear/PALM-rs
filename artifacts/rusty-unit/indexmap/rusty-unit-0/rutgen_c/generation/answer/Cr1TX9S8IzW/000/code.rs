// Answer 0

#[test]
fn test_key() {
    struct MockIndices;
    struct MockEntries<K, V>(Vec<(K, V)>);

    let key = "test_key";
    let map = RefMut {
        indices: &mut MockIndices,
        entries: &mut MockEntries(Vec::new()),
    };

    let vacant_entry = VacantEntry {
        map,
        hash: HashValue(0),
        key,
    };

    assert_eq!(vacant_entry.key(), &"test_key");
}


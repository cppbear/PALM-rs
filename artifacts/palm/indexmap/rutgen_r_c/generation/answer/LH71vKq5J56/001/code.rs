// Answer 0

#[test]
fn test_index_empty_indices() {
    let mut indices = Vec::new();
    let mut entries = Vec::new();
    let ref_mut = RefMut { indices: &mut indices, entries: &mut entries };
    let hash_value = HashValue(123);
    let key = "test_key";
    let vacant_entry = VacantEntry { map: ref_mut, hash: hash_value, key: key };

    assert_eq!(vacant_entry.index(), 0);
}

#[test]
fn test_index_non_empty_indices() {
    let mut indices = vec![1, 2, 3];
    let mut entries = Vec::new();
    let ref_mut = RefMut { indices: &mut indices, entries: &mut entries };
    let hash_value = HashValue(456);
    let key = "another_key";
    let vacant_entry = VacantEntry { map: ref_mut, hash: hash_value, key: key };

    assert_eq!(vacant_entry.index(), 3);
}

#[test]
fn test_index_with_multiple_inserts() {
    let mut indices = vec![1, 2, 3, 4, 5];
    let mut entries = Vec::new();
    let ref_mut = RefMut { indices: &mut indices, entries: &mut entries };
    let hash_value = HashValue(789);
    let key = "yet_another_key";
    let vacant_entry = VacantEntry { map: ref_mut, hash: hash_value, key: key };

    assert_eq!(vacant_entry.index(), 5);
}


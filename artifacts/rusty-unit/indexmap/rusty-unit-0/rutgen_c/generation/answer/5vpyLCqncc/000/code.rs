// Answer 0

#[test]
fn test_shift_insert_hashed_nocheck_valid_insertion() {
    struct TestIndices {
        data: Vec<usize>,
    }

    struct TestEntries<K, V> {
        data: Vec<Bucket<K, V>>,
    }

    let mut indices = TestIndices { data: vec![0, 1] };
    let mut entries: TestEntries<u32, String> = TestEntries { data: Vec::new() };
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let raw_entry = RawVacantEntryMut { map: ref_mut, hash_builder: &() };

    let (key_ref, value_ref) = raw_entry.shift_insert_hashed_nocheck(0, 42, 1u32, "test".to_string());

    assert_eq!(*key_ref, 1);
    assert_eq!(*value_ref, "test".to_string());
}

#[test]
#[should_panic]
fn test_shift_insert_hashed_nocheck_index_out_of_bounds() {
    struct TestIndices {
        data: Vec<usize>,
    }

    struct TestEntries<K, V> {
        data: Vec<Bucket<K, V>>,
    }

    let mut indices = TestIndices { data: vec![0] };
    let mut entries: TestEntries<u32, String> = TestEntries { data: Vec::new() };
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let raw_entry = RawVacantEntryMut { map: ref_mut, hash_builder: &() };

    raw_entry.shift_insert_hashed_nocheck(2, 42, 1u32, "test".to_string());
}


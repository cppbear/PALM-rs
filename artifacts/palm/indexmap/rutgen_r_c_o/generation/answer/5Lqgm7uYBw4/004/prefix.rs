// Answer 0

fn test_swap_indices_equal_within_bounds() {
    let mut indices = hash_table::HashTable::new();
    let value1 = HashValue(1);
    let value2 = HashValue(2);
    let entry1 = Bucket { hash: value1, key: "key1", value: "value1" };
    let entry2 = Bucket { hash: value2, key: "key2", value: "value2" };
    let mut entries = vec![entry1, entry2];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.swap_indices(0, 0);
}

fn test_swap_indices_equal_out_of_bounds() {
    let mut indices = hash_table::HashTable::new();
    let value1 = HashValue(1);
    let entry1 = Bucket { hash: value1, key: "key1", value: "value1" };
    let mut entries = vec![entry1];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.swap_indices(1, 1);
}

fn test_swap_indices_different_bounds() {
    let mut indices = hash_table::HashTable::new();
    let value1 = HashValue(1);
    let value2 = HashValue(2);
    let entry1 = Bucket { hash: value1, key: "key1", value: "value1" };
    let entry2 = Bucket { hash: value2, key: "key2", value: "value2" };
    let mut entries = vec![entry1, entry2];

    indices.insert(value1.get(), 0);
    indices.insert(value2.get(), 1);

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.swap_indices(0, 1);
}

fn test_swap_indices_invalid_indices() {
    let mut indices = hash_table::HashTable::new();
    let value1 = HashValue(1);
    let value2 = HashValue(2);
    let entry1 = Bucket { hash: value1, key: "key1", value: "value1" };
    let entry2 = Bucket { hash: value2, key: "key2", value: "value2" };
    let mut entries = vec![entry1, entry2];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    indices.insert(value1.get(), 0);
    indices.insert(value2.get(), 2); // Invalid index

    ref_mut.swap_indices(0, 1);
}

fn test_swap_indices_empty_entries() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<&str, &str>> = Vec::new();

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    // Shouldn't panic but should not do anything since entries are empty
    ref_mut.swap_indices(0, 0);
}


// Answer 0

#[test]
fn test_shift_insert_unique_with_index_at_end() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<String, i32>> = Vec::with_capacity(5);
    let index = entries.len(); // index == end, should be valid

    let hash = HashValue(1);
    let key = String::from("key1");
    let value = 42;

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.shift_insert_unique(index, hash, key, value);
}

#[test]
fn test_shift_insert_unique_with_index_in_middle() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<String, i32>> = Vec::with_capacity(5);
    entries.push(Bucket { hash: HashValue(0), key: String::from("key0"), value: 41 });
    
    let index = 1; // Valid index
    let hash = HashValue(1);
    let key = String::from("key1");
    let value = 42;

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.shift_insert_unique(index, hash, key, value);
}

#[test]
fn test_shift_insert_unique_with_capacity_check() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<String, i32>> = Vec::with_capacity(1); // capacity is 1

    let index = 0; // Valid index
    let hash = HashValue(1);
    let key = String::from("key1");
    let value = 42;

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.shift_insert_unique(index, hash, key, value);
    
    let index2 = 0; // Overwriting the same index should be valid now
    let hash2 = HashValue(2);
    let key2 = String::from("key2");
    let value2 = 43;

    ref_mut.shift_insert_unique(index2, hash2, key2, value2);
}


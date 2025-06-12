// Answer 0

#[test]
fn test_swap_remove_index_valid() {
    let mut indices = hash_table::HashTable::new();
    let mut entries = vec![
        Bucket { hash: HashValue(1), key: 1, value: "value1" },
        Bucket { hash: HashValue(2), key: 2, value: "value2" },
        Bucket { hash: HashValue(3), key: 3, value: "value3" },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let result = ref_mut.swap_remove_index(1);
}

#[test]
fn test_swap_remove_index_first_entry() {
    let mut indices = hash_table::HashTable::new();
    let mut entries = vec![
        Bucket { hash: HashValue(4), key: 4, value: "value4" },
        Bucket { hash: HashValue(5), key: 5, value: "value5" },
        Bucket { hash: HashValue(6), key: 6, value: "value6" },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let result = ref_mut.swap_remove_index(0);
}

#[test]
fn test_swap_remove_index_last_entry() {
    let mut indices = hash_table::HashTable::new();
    let mut entries = vec![
        Bucket { hash: HashValue(7), key: 7, value: "value7" },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let result = ref_mut.swap_remove_index(0);
}

#[test]
fn test_swap_remove_index_middle_entry() {
    let mut indices = hash_table::HashTable::new();
    let mut entries = vec![
        Bucket { hash: HashValue(8), key: 8, value: "value8" },
        Bucket { hash: HashValue(9), key: 9, value: "value9" },
        Bucket { hash: HashValue(10), key: 10, value: "value10" },
        Bucket { hash: HashValue(11), key: 11, value: "value11" },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let result = ref_mut.swap_remove_index(2);
}


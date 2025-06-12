// Answer 0

#[test]
fn test_insert_bulk_no_grow_with_enough_capacity() {
    let mut indices: Indices = hash_table::HashTable::with_capacity(10);
    let entries = vec![
        Bucket { hash: HashValue(1), key: 10, value: "a" },
        Bucket { hash: HashValue(2), key: 20, value: "b" },
    ];
    insert_bulk_no_grow(&mut indices, &entries);
    assert_eq!(indices.len(), 2);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_insert_bulk_no_grow_with_insufficient_capacity() {
    let mut indices: Indices = hash_table::HashTable::with_capacity(1); // capacity is 1
    let entries = vec![
        Bucket { hash: HashValue(1), key: 10, value: "a" },
        Bucket { hash: HashValue(2), key: 20, value: "b" },
    ]; // trying to insert 2 entries
    insert_bulk_no_grow(&mut indices, &entries);
}

#[test]
fn test_insert_bulk_no_grow_with_exact_capacity() {
    let mut indices: Indices = hash_table::HashTable::with_capacity(2);
    let entries = vec![
        Bucket { hash: HashValue(1), key: 10, value: "a" },
        Bucket { hash: HashValue(2), key: 20, value: "b" },
    ]; // exactly 2 entries
    insert_bulk_no_grow(&mut indices, &entries);
    assert_eq!(indices.len(), 2);
}


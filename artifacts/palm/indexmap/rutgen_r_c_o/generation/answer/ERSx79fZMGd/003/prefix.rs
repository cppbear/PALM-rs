// Answer 0

#[test]
#[should_panic]
fn test_insert_bulk_no_grow_panic_case() {
    let mut indices: Indices = hash_table::HashTable::with_capacity(2);
    let entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 10, value: 100 },
        Bucket { hash: HashValue(2), key: 20, value: 200 },
        Bucket { hash: HashValue(3), key: 30, value: 300 },
    ];
    insert_bulk_no_grow(&mut indices, &entries);
}

#[test]
fn test_insert_bulk_no_grow_normal_case() {
    let mut indices: Indices = hash_table::HashTable::with_capacity(3);
    let entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 10, value: 100 },
        Bucket { hash: HashValue(2), key: 20, value: 200 },
    ];
    insert_bulk_no_grow(&mut indices, &entries);
}

#[test]
#[should_panic]
fn test_insert_bulk_no_grow_exceed_capacity() {
    let mut indices: Indices = hash_table::HashTable::with_capacity(1);
    let entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 10, value: 100 },
        Bucket { hash: HashValue(2), key: 20, value: 200 },
    ];
    insert_bulk_no_grow(&mut indices, &entries);
}


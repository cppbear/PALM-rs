// Answer 0

#[test]
fn test_swap_indices_equal_within_bounds() {
    let mut indices = hash_table::HashTable::new();
    let mut entries = Vec::with_capacity(5);
    entries.push(Bucket { hash: HashValue(0), key: 1, value: 10 });
    entries.push(Bucket { hash: HashValue(1), key: 2, value: 20 });
    entries.push(Bucket { hash: HashValue(2), key: 3, value: 30 });
    entries.push(Bucket { hash: HashValue(3), key: 4, value: 40 });
    entries.push(Bucket { hash: HashValue(4), key: 5, value: 50 });

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    ref_mut.swap_indices(2, 2);
}

#[test]
fn test_swap_indices_same_index_first_entry() {
    let mut indices = hash_table::HashTable::new();
    let mut entries = Vec::with_capacity(3);
    entries.push(Bucket { hash: HashValue(0), key: 1, value: 10 });
    entries.push(Bucket { hash: HashValue(1), key: 2, value: 20 });
    entries.push(Bucket { hash: HashValue(2), key: 3, value: 30 });

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    ref_mut.swap_indices(0, 0);
}

#[test]
fn test_swap_indices_same_index_middle_entry() {
    let mut indices = hash_table::HashTable::new();
    let mut entries = Vec::with_capacity(4);
    entries.push(Bucket { hash: HashValue(0), key: 1, value: 10 });
    entries.push(Bucket { hash: HashValue(1), key: 2, value: 20 });
    entries.push(Bucket { hash: HashValue(2), key: 3, value: 30 });
    entries.push(Bucket { hash: HashValue(3), key: 4, value: 40 });

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    ref_mut.swap_indices(1, 1);
}

#[test]
fn test_swap_indices_same_index_last_entry() {
    let mut indices = hash_table::HashTable::new();
    let mut entries = Vec::with_capacity(6);
    entries.push(Bucket { hash: HashValue(0), key: 1, value: 10 });
    entries.push(Bucket { hash: HashValue(1), key: 2, value: 20 });
    entries.push(Bucket { hash: HashValue(2), key: 3, value: 30 });
    entries.push(Bucket { hash: HashValue(3), key: 4, value: 40 });
    entries.push(Bucket { hash: HashValue(4), key: 5, value: 50 });
    entries.push(Bucket { hash: HashValue(5), key: 6, value: 60 });

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    ref_mut.swap_indices(5, 5);
}

#[test]
#[should_panic]
fn test_swap_indices_panic_out_of_bounds() {
    let mut indices = hash_table::HashTable::new();
    let mut entries = Vec::with_capacity(2);
    entries.push(Bucket { hash: HashValue(0), key: 1, value: 10 });
    entries.push(Bucket { hash: HashValue(1), key: 2, value: 20 });

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    ref_mut.swap_indices(0, 2);
}


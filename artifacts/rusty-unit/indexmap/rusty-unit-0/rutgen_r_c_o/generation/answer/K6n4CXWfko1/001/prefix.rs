// Answer 0

#[test]
fn test_reserve_entries_minimum() {
    let mut indices = hash_table::HashTable::with_capacity(10);
    let mut entries: Vec<Bucket<usize, usize>> = Vec::new();
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    ref_mut.reserve_entries(0);
}

#[test]
fn test_reserve_entries_exact_capacity() {
    let mut indices = hash_table::HashTable::with_capacity(10);
    let mut entries: Vec<Bucket<usize, usize>> = Vec::new();
    entries.reserve_exact(5); // existing capacity is 5
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    ref_mut.reserve_entries(5);
}

#[test]
fn test_reserve_entries_below_capacity() {
    let mut indices = hash_table::HashTable::with_capacity(10);
    let mut entries: Vec<Bucket<usize, usize>> = Vec::new();
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    ref_mut.reserve_entries(3);
}

#[test]
fn test_reserve_entries_edge_case() {
    let mut indices = hash_table::HashTable::with_capacity(20);
    let mut entries: Vec<Bucket<usize, usize>> = Vec::new();
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    ref_mut.reserve_entries(19); // testing just below the capacity
}

#[should_panic]
fn test_reserve_entries_exceeding_capacity() {
    let mut indices = hash_table::HashTable::with_capacity(5);
    let mut entries: Vec<Bucket<usize, usize>> = Vec::new();
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    ref_mut.reserve_entries(6); // should trigger panic as it exceeds capacity
}


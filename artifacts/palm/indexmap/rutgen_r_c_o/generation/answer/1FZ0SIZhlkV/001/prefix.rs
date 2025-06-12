// Answer 0

#[test]
fn test_shift_remove_index_valid() {
    let mut indices = hash_table::HashTable::default();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 10 },
        Bucket { hash: HashValue(2), key: 2, value: 20 },
        Bucket { hash: HashValue(3), key: 3, value: 30 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    let result = ref_mut.shift_remove_index(1);
}

#[test]
fn test_shift_remove_index_first_entry() {
    let mut indices = hash_table::HashTable::default();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 10 },
        Bucket { hash: HashValue(2), key: 2, value: 20 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    let result = ref_mut.shift_remove_index(0);
}

#[test]
fn test_shift_remove_index_last_entry() {
    let mut indices = hash_table::HashTable::default();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 10 },
        Bucket { hash: HashValue(2), key: 2, value: 20 },
        Bucket { hash: HashValue(3), key: 3, value: 30 },
        Bucket { hash: HashValue(4), key: 4, value: 40 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    let result = ref_mut.shift_remove_index(3);
}

#[test]
fn test_shift_remove_index_middle_entry() {
    let mut indices = hash_table::HashTable::default();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 10 },
        Bucket { hash: HashValue(2), key: 2, value: 20 },
        Bucket { hash: HashValue(3), key: 3, value: 30 },
        Bucket { hash: HashValue(4), key: 4, value: 40 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    let result = ref_mut.shift_remove_index(1);
}

#[test]
fn test_shift_remove_index_out_of_bounds() {
    let mut indices = hash_table::HashTable::default();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 10 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    let result = ref_mut.shift_remove_index(1);
}


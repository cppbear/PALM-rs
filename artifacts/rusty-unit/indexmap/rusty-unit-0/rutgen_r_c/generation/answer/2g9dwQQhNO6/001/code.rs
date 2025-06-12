// Answer 0

#[test]
fn test_swap_remove_index_valid_case() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(0), key: 1, value: 10 },
        Bucket { hash: HashValue(1), key: 2, value: 20 },
        Bucket { hash: HashValue(2), key: 3, value: 30 },
    ];
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    let result = ref_mut.swap_remove_index(1);
    
    assert_eq!(result, Some((2, 20)));
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[1], Bucket { hash: HashValue(2), key: 3, value: 30 });
}

#[test]
fn test_swap_remove_index_invalid_case() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(0), key: 1, value: 10 },
        Bucket { hash: HashValue(1), key: 2, value: 20 },
        Bucket { hash: HashValue(2), key: 3, value: 30 },
    ];
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    let result = ref_mut.swap_remove_index(3);
    
    assert_eq!(result, None);
}


// Answer 0

#[test]
fn test_swap_remove_index_valid() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 10 },
        Bucket { hash: HashValue(2), key: 2, value: 20 },
        Bucket { hash: HashValue(3), key: 3, value: 30 },
    ];
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    if let Some((key, value)) = ref_mut.swap_remove_index(1) {
        assert_eq!(key, 2);
        assert_eq!(value, 20);
        assert_eq!(entries.len(), 2);
    } else {
        panic!("Expected to remove an entry successfully");
    }
}

#[test]
fn test_swap_remove_index_out_of_bounds() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 10 },
        Bucket { hash: HashValue(2), key: 2, value: 20 },
    ];
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    // Trying to remove an entry at an out-of-bounds index
    let result = ref_mut.swap_remove_index(10);
    assert_eq!(result, None);
}

#[test]
fn test_swap_remove_index_first_entry() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 10 },
        Bucket { hash: HashValue(2), key: 2, value: 20 },
    ];
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    if let Some((key, value)) = ref_mut.swap_remove_index(0) {
        assert_eq!(key, 1);
        assert_eq!(value, 10);
        assert_eq!(entries.len(), 1);
    } else {
        panic!("Expected to remove the first entry successfully");
    }
}


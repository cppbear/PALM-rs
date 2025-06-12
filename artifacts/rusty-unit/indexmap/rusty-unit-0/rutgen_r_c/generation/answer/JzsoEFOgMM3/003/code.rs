// Answer 0

#[test]
fn test_decrement_indices_large_shift() {
    let mut indices = hashbrown::HashTable::with_capacity(10);
    indices.insert(2, 0);
    indices.insert(3, 1);
    indices.insert(4, 2);
    indices.insert(5, 3);
    indices.insert(6, 4);
    
    let bucket1 = Bucket {
        hash: HashValue(1),
        key: "key1",
        value: "value1",
    };

    let bucket2 = Bucket {
        hash: HashValue(2),
        key: "key2",
        value: "value2",
    };

    let entries = vec![bucket1, bucket2];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    ref_mut.decrement_indices(1, 2);
    
    assert_eq!(indices.get(&1), None);
    assert_eq!(indices.get(&2).unwrap(), &1);
}

#[test]
fn test_decrement_indices_small_shift() {
    let mut indices = hashbrown::HashTable::with_capacity(10);
    indices.insert(0, 0);
    indices.insert(1, 1);
    
    let bucket = Bucket {
        hash: HashValue(1),
        key: "key",
        value: "value",
    };

    let entries = vec![bucket];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    ref_mut.decrement_indices(0, 1);
    
    assert_eq!(indices.get(&0), None);
}

#[test]
#[should_panic]
fn test_decrement_indices_panic_on_invalid_range() {
    let mut indices = hashbrown::HashTable::with_capacity(10);
    indices.insert(2, 0);
    indices.insert(3, 1);
    
    let bucket1 = Bucket {
        hash: HashValue(1),
        key: "key1",
        value: "value1",
    };

    let bucket2 = Bucket {
        hash: HashValue(2),
        key: "key2",
        value: "value2",
    };

    let entries = vec![bucket1, bucket2];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    ref_mut.decrement_indices(3, 4); // This should panic since the range is invalid
}


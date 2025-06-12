// Answer 0

#[test]
fn test_move_index_valid_movement() {
    let mut indices = hashbrown::HashMap::<usize, usize>::default();
    let mut entries = vec![
        Bucket { hash: HashValue(1), key: 1, value: "a" },
        Bucket { hash: HashValue(2), key: 2, value: "b" },
        Bucket { hash: HashValue(3), key: 3, value: "c" },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    ref_mut.move_index(1, 2);
    
    assert_eq!(entries[1].key, 3);
    assert_eq!(entries[2].key, 2);
}

#[test]
#[should_panic]
fn test_move_index_invalid_from_index() {
    let mut indices = hashbrown::HashMap::<usize, usize>::default();
    let mut entries = vec![
        Bucket { hash: HashValue(1), key: 1, value: "a" },
        Bucket { hash: HashValue(2), key: 2, value: "b" },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    ref_mut.move_index(2, 1); // 'from' index out of bounds
}

#[test]
#[should_panic]
fn test_move_index_invalid_to_index() {
    let mut indices = hashbrown::HashMap::<usize, usize>::default();
    let mut entries = vec![
        Bucket { hash: HashValue(1), key: 1, value: "a" },
        Bucket { hash: HashValue(2), key: 2, value: "b" },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    ref_mut.move_index(0, 2); // 'to' index out of bounds
}

#[test]
#[should_panic]
fn test_move_index_same_index() {
    let mut indices = hashbrown::HashMap::<usize, usize>::default();
    let mut entries = vec![
        Bucket { hash: HashValue(1), key: 1, value: "a" },
        Bucket { hash: HashValue(2), key: 2, value: "b" },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    ref_mut.move_index(1, 1); // 'from' and 'to' are the same
}

#[test]
fn test_move_index_reverse_movement() {
    let mut indices = hashbrown::HashMap::<usize, usize>::default();
    let mut entries = vec![
        Bucket { hash: HashValue(1), key: 1, value: "a" },
        Bucket { hash: HashValue(2), key: 2, value: "b" },
        Bucket { hash: HashValue(3), key: 3, value: "c" },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    ref_mut.move_index(2, 1);
    
    assert_eq!(entries[1].key, 3);
    assert_eq!(entries[2].key, 2);
}


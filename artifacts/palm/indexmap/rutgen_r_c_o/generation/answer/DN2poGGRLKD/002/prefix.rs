// Answer 0

#[test]
fn test_increment_indices_case1() {
    let mut indices = hashbrown::HashTable::with_capacity(3);
    indices.insert(2);
    indices.insert(3);
    indices.insert(4);
    
    let entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 10 },
        Bucket { hash: HashValue(2), key: 2, value: 20 },
    ];
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.increment_indices(0, 2);
}

#[test]
fn test_increment_indices_case2() {
    let mut indices = hashbrown::HashTable::with_capacity(3);
    indices.insert(2);
    indices.insert(3);
    indices.insert(4);
    
    let entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 3, value: 30 },
        Bucket { hash: HashValue(4), key: 4, value: 40 },
    ];
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.increment_indices(0, 2);
}


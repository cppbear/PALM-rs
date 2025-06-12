// Answer 0

#[test]
fn test_decrement_indices_case1() {
    let mut indices = hash_table::HashTable::with_capacity(10);
    for i in 0..5 {
        indices.insert(i);
    }
    let entries: Vec<Bucket<usize, usize>> = (0..5).map(|i| Bucket {
        hash: HashValue(i),
        key: i,
        value: i * 10,
    }).collect();
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.decrement_indices(0, 5);
}

#[test]
fn test_decrement_indices_case2() {
    let mut indices = hash_table::HashTable::with_capacity(10);
    for i in 0..7 {
        indices.insert(i);
    }
    let entries: Vec<Bucket<usize, usize>> = (0..7).map(|i| Bucket {
        hash: HashValue(i),
        key: i,
        value: i * 10,
    }).collect();
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.decrement_indices(0, 7);
}

#[test]
fn test_decrement_indices_case3() {
    let mut indices = hash_table::HashTable::with_capacity(10);
    for i in 0..4 {
        indices.insert(i);
    }
    let entries: Vec<Bucket<usize, usize>> = (0..4).map(|i| Bucket {
        hash: HashValue(i),
        key: i,
        value: i * 10,
    }).collect();
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.decrement_indices(0, 4);
}


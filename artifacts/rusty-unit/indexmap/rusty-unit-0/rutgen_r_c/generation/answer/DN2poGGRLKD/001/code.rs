// Answer 0

fn test_increment_indices_large_shift() {
    let mut indices = hashbrown::HashMap::new();
    for i in 0..10 {
        indices.insert(i, i);
    }
    let entries: Vec<Bucket<usize, usize>> = (0..10).map(|i| Bucket { hash: HashValue(i), key: i, value: i }).collect();
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.increment_indices(0, 5);

    for (i, &value) in indices.iter().enumerate() {
        if i < 5 {
            assert_eq!(value, i);
        } else {
            assert_eq!(value, i - 1);
        }
    }
}

fn test_increment_indices_small_shift() {
    let mut indices = hashbrown::HashMap::new();
    for i in 0..5 {
        indices.insert(i, i);
    }
    
    let entries: Vec<Bucket<usize, usize>> = (0..5).map(|i| Bucket { hash: HashValue(i), key: i, value: i }).collect();
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.increment_indices(0, 3);

    for (i, &value) in indices.iter().enumerate() {
        if i < 3 {
            assert_eq!(value, i);
        } else {
            assert_eq!(value, i - 1);
        }
    }
}

fn test_increment_indices_zero_length() {
    let mut indices = hashbrown::HashMap::new();
    for i in 0..10 {
        indices.insert(i, i);
    }
    
    let entries: Vec<Bucket<usize, usize>> = (0..10).map(|i| Bucket { hash: HashValue(i), key: i, value: i }).collect();
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.increment_indices(5, 5);

    for (i, &value) in indices.iter().enumerate() {
        assert_eq!(value, i);
    }
}

fn test_increment_indices_exceed_capacity() {
    let mut indices = hashbrown::HashMap::new();
    for i in 0..8 {
        indices.insert(i, i);
    }
    
    let entries: Vec<Bucket<usize, usize>> = (0..8).map(|i| Bucket { hash: HashValue(i), key: i, value: i }).collect();
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.increment_indices(0, 5);
    
    for (i, &value) in indices.iter().enumerate() {
        if i < 5 {
            assert_eq!(value, i);
        } else {
            assert_eq!(value, i - 1);
        }
    }
}

fn test_increment_indices_out_of_range() {
    let mut indices = hashbrown::HashMap::new();
    for i in 0..10 {
        indices.insert(i, i);
    }
    
    let entries: Vec<Bucket<usize, usize>> = (0..10).map(|i| Bucket { hash: HashValue(i), key: i, value: i }).collect();
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.increment_indices(8, 10);

    for (i, &value) in indices.iter().enumerate() {
        if i < 8 {
            assert_eq!(value, i);
        } else {
            assert_eq!(value, i);
        }
    }
}


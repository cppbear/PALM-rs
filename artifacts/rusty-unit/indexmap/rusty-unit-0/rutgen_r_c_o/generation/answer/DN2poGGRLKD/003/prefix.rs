// Answer 0

#[test]
fn test_increment_indices_with_large_shift() {
    let mut indices = hashbrown::HashMap::new();
    indices.insert(0, 0);
    indices.insert(1, 1);
    indices.insert(2, 2);
    indices.insert(3, 3);
    indices.insert(4, 4);
    indices.insert(5, 5);

    let entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 10 },
        Bucket { hash: HashValue(2), key: 2, value: 20 },
        Bucket { hash: HashValue(3), key: 3, value: 30 },
    ];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.increment_indices(0, 2);
}

#[test]
fn test_increment_indices_with_middle_shift() {
    let mut indices = hashbrown::HashMap::new();
    indices.insert(0, 0);
    indices.insert(1, 1);
    indices.insert(2, 2);
    indices.insert(3, 3);

    let entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 10 },
        Bucket { hash: HashValue(2), key: 2, value: 20 },
        Bucket { hash: HashValue(3), key: 3, value: 30 },
        Bucket { hash: HashValue(4), key: 4, value: 40 },
    ];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.increment_indices(1, 3);
}

#[test]
fn test_increment_indices_no_shift() {
    let mut indices = hashbrown::HashMap::new();
    indices.insert(0, 0);
    indices.insert(1, 1);
    indices.insert(2, 2);
    indices.insert(3, 3);

    let entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 10 },
        Bucket { hash: HashValue(2), key: 2, value: 20 },
        Bucket { hash: HashValue(3), key: 3, value: 30 },
        Bucket { hash: HashValue(4), key: 4, value: 40 },
        Bucket { hash: HashValue(5), key: 5, value: 50 },
    ];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.increment_indices(3, 4);
}

#[test]
#[should_panic]
fn test_increment_indices_panic_on_invalid_range() {
    let mut indices = hashbrown::HashMap::new();
    indices.insert(0, 0);
    indices.insert(1, 1);
    indices.insert(2, 2);

    let entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 10 },
        Bucket { hash: HashValue(2), key: 2, value: 20 },
    ];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.increment_indices(1, 3);
}


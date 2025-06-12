// Answer 0

#[test]
fn test_increment_indices_large_shift() {
    let mut indices = vec![0, 1, 2, 3, 4].into_boxed_slice();
    let entries = vec![
        Bucket { hash: HashValue(1), key: 1, value: "A" },
        Bucket { hash: HashValue(2), key: 2, value: "B" },
        Bucket { hash: HashValue(3), key: 3, value: "C" },
        Bucket { hash: HashValue(4), key: 4, value: "D" },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    // Triggering the large shift case
    ref_mut.increment_indices(0, 4);

    assert_eq!(&*indices, &[1, 2, 3, 4, 4]);
}

#[test]
fn test_increment_indices_small_shift() {
    let mut indices = vec![0, 1, 2, 3].into_boxed_slice();
    let entries = vec![
        Bucket { hash: HashValue(1), key: 1, value: "A" },
        Bucket { hash: HashValue(2), key: 2, value: "B" },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    // Ensure shifted_entries.len() == indices.capacity() / 2
    ref_mut.increment_indices(0, 2);

    assert_eq!(&*indices, &[1, 2, 3, 3]);
}

#[should_panic(expected = "index out of bounds")]
#[test]
fn test_increment_indices_out_of_bounds() {
    let mut indices = vec![0, 1, 2].into_boxed_slice();
    let entries = vec![
        Bucket { hash: HashValue(1), key: 1, value: "A" },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    // Attempt to increment beyond the length of entries
    ref_mut.increment_indices(0, 5);
}

#[test]
fn test_increment_indices_no_shift() {
    let mut indices = vec![0, 1, 2, 3].into_boxed_slice();
    let entries = vec![
        Bucket { hash: HashValue(1), key: 1, value: "A" },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    // No increment because end is inclusive
    ref_mut.increment_indices(0, 0);

    assert_eq!(&*indices, &[0, 1, 2, 3]);
}


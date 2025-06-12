// Answer 0

#[test]
fn test_shift_remove_index_should_return_none_when_index_out_of_bounds() {
    // Setup test data
    let mut indices = Indices::default();
    let mut entries: Vec<Bucket<usize, usize>> = vec![];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    // Invoke shift_remove_index with an out-of-bounds index
    let result = ref_mut.shift_remove_index(0); // Here, the vector is empty

    // Assert the expected result
    assert_eq!(result, None);
}

#[test]
fn test_shift_remove_index_should_return_none_when_index_is_negative() {
    // Setup test data
    let mut indices = Indices::default();
    let mut entries: Vec<Bucket<usize, usize>> = vec![];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    // Invoke shift_remove_index with a negative index (simulated by using usize max)
    let result = ref_mut.shift_remove_index(usize::MAX);

    // Assert the expected result
    assert_eq!(result, None);
}


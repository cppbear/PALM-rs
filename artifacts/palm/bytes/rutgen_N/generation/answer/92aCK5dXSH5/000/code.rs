// Answer 0

#[test]
fn test_rebuild_vec() {
    let mut data: Vec<u8> = vec![1, 2, 3, 4, 5];
    let len = 5;
    let cap = 5;
    let off = 0;

    // Obtain a raw pointer to the data.
    let ptr = data.as_mut_ptr();

    // Make the raw pointer mutable and call the unsafe function.
    let reconstructed_vec = unsafe { rebuild_vec(ptr, len, cap, off) };

    // Verify the contents of the reconstructed vector.
    assert_eq!(reconstructed_vec.len(), 5);
    assert_eq!(reconstructed_vec, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_rebuild_vec_with_offset() {
    let mut data: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let len = 10;
    let cap = 10;
    let off = 2;

    // Obtain a raw pointer to the data.
    let ptr = data.as_mut_ptr();

    // Call the unsafe function with an offset.
    let reconstructed_vec = unsafe { rebuild_vec(ptr, len, cap, off) };

    // Verify the contents of the reconstructed vector.
    assert_eq!(reconstructed_vec.len(), 12); // 10 + 2 offset
    for i in 0..10 {
        assert_eq!(reconstructed_vec[i], i as u8 + 1);
    }
}

#[should_panic]
#[test]
fn test_rebuild_vec_with_invalid_offset() {
    let mut data: Vec<u8> = vec![1, 2, 3, 4, 5];
    let len = 5;
    let cap = 5;
    let off = 10; // Invalid offset that exceeds the current length.

    // Obtain a raw pointer to the data.
    let ptr = data.as_mut_ptr();

    // Call the unsafe function with an invalid offset.
    unsafe { rebuild_vec(ptr, len, cap, off) };
}


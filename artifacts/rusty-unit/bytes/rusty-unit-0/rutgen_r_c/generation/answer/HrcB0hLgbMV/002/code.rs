// Answer 0

#[test]
fn test_chunk_mut_non_full_vec() {
    let mut vec: Vec<u8> = Vec::with_capacity(64);
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let uninit_slice: &mut UninitSlice = vec.chunk_mut();
    assert_eq!(uninit_slice.0.len(), vec.capacity() - vec.len());
}

#[test]
fn test_chunk_mut_empty_vec() {
    let mut vec: Vec<u8> = Vec::with_capacity(64);

    let uninit_slice: &mut UninitSlice = vec.chunk_mut();
    assert_eq!(uninit_slice.0.len(), vec.capacity() - vec.len());
}

#[test]
fn test_chunk_mut_reserve_space() {
    let mut vec: Vec<u8> = Vec::new();
    vec.reserve(3); // Ensure capacity is 3

    let uninit_slice: &mut UninitSlice = vec.chunk_mut();
    assert_eq!(uninit_slice.0.len(), vec.capacity() - vec.len());
}


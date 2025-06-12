// Answer 0

#[test]
fn test_chunk_mut_empty_vector() {
    let mut vec: Vec<u8> = Vec::new();
    let chunk: &mut UninitSlice = vec.chunk_mut();
    assert_eq!(chunk.len(), vec.capacity());
}

#[test]
fn test_chunk_mut_full_vector() {
    let mut vec: Vec<u8> = vec![0; 64];
    let chunk: &mut UninitSlice = vec.chunk_mut();
    assert_eq!(chunk.len(), 64);
    assert!(vec.capacity() > 64); // Capacity should grow
}

#[test]
fn test_chunk_mut_after_adding_elements() {
    let mut vec: Vec<u8> = Vec::with_capacity(64);
    vec.push(1);
    vec.push(2);
    let chunk: &mut UninitSlice = vec.chunk_mut();
    assert_eq!(chunk.len(), vec.capacity() - 2); // It should have the remaining capacity
}

#[test]
fn test_chunk_mut_multiple_resizes() {
    let mut vec: Vec<u8> = Vec::with_capacity(10);
    for _ in 0..12 {
        vec.push(0); // This will trigger multiple resizes
    }
    let chunk: &mut UninitSlice = vec.chunk_mut();
    assert!(chunk.len() > 0); // Ensure that there is remaining capacity
}


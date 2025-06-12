// Answer 0

#[test]
fn test_copy_to_bytes_with_zero_length() {
    let mut bytes_mut = BytesMut::new();
    bytes_mut.copy_to_bytes(0);
}

#[test]
fn test_copy_to_bytes_with_one_length() {
    let mut bytes_mut = BytesMut::with_capacity(1);
    bytes_mut.extend_from_slice(&[1]);
    bytes_mut.copy_to_bytes(1);
}

#[test]
fn test_copy_to_bytes_with_small_length() {
    let mut bytes_mut = BytesMut::with_capacity(5);
    bytes_mut.extend_from_slice(&[1, 2, 3, 4, 5]);
    bytes_mut.copy_to_bytes(3);
}

#[test]
fn test_copy_to_bytes_with_full_length() {
    let mut bytes_mut = BytesMut::with_capacity(5);
    bytes_mut.extend_from_slice(&[1, 2, 3, 4, 5]);
    bytes_mut.copy_to_bytes(5);
}

#[test]
#[should_panic]
fn test_copy_to_bytes_with_exceeding_length() {
    let mut bytes_mut = BytesMut::with_capacity(3);
    bytes_mut.extend_from_slice(&[1, 2, 3]);
    bytes_mut.copy_to_bytes(4);
}

#[test]
fn test_copy_to_bytes_with_max_vector_position() {
    let mut bytes_mut = BytesMut::with_capacity(MAX_VEC_POS);
    let vec_data = vec![0; MAX_VEC_POS];
    bytes_mut.extend_from_slice(&vec_data);
    bytes_mut.copy_to_bytes(MAX_VEC_POS);
}


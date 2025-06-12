// Answer 0

#[test]
fn test_from_vec_empty() {
    let vec: Vec<u8> = Vec::new();
    let bytes_mut = BytesMut::from_vec(vec);
    assert_eq!(bytes_mut.len(), 0);
    assert_eq!(bytes_mut.capacity(), 0);
}

#[test]
fn test_from_vec_non_empty() {
    let vec: Vec<u8> = vec![1, 2, 3, 4, 5];
    let bytes_mut = BytesMut::from_vec(vec);
    assert_eq!(bytes_mut.len(), 5);
    assert_eq!(bytes_mut.capacity(), 5);
}

#[test]
fn test_from_vec_capacity() {
    let vec: Vec<u8> = vec![0; 10];
    let bytes_mut = BytesMut::from_vec(vec);
    assert_eq!(bytes_mut.len(), 10);
    assert_eq!(bytes_mut.capacity(), 10);
}


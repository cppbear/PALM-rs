// Answer 0

#[test]
fn test_from_vec_empty() {
    let vec = Vec::new();
    let bytes_mut = BytesMut::from_vec(vec);
    assert_eq!(bytes_mut.len(), 0);
    assert_eq!(bytes_mut.cap, 0);
}

#[test]
fn test_from_vec_non_empty() {
    let vec = vec![1, 2, 3, 4, 5];
    let bytes_mut = BytesMut::from_vec(vec);
    assert_eq!(bytes_mut.len(), 5);
    assert_eq!(bytes_mut.cap, 5);
}

#[test]
fn test_from_vec_capacity() {
    let vec = Vec::with_capacity(10);
    let bytes_mut = BytesMut::from_vec(vec);
    assert_eq!(bytes_mut.len(), 0);
    assert_eq!(bytes_mut.cap, 10);
}

#[test]
fn test_from_vec_large_capacity() {
    let vec = vec![0; 1024];
    let bytes_mut = BytesMut::from_vec(vec);
    assert_eq!(bytes_mut.len(), 1024);
    assert_eq!(bytes_mut.cap, 1024);
}

#[test]
fn test_from_vec_single_element() {
    let vec = vec![42];
    let bytes_mut = BytesMut::from_vec(vec);
    assert_eq!(bytes_mut.len(), 1);
    assert_eq!(bytes_mut.cap, 1);
}


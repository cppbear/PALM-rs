// Answer 0

#[test]
fn test_from_vec_empty() {
    let vec: Vec<u8> = Vec::new();
    let _ = BytesMut::from_vec(vec);
}

#[test]
fn test_from_vec_small() {
    let vec: Vec<u8> = vec![1, 2, 3];
    let _ = BytesMut::from_vec(vec);
}

#[test]
fn test_from_vec_medium() {
    let vec: Vec<u8> = (0..100).map(|x| x as u8).collect();
    let _ = BytesMut::from_vec(vec);
}

#[test]
fn test_from_vec_large() {
    let vec: Vec<u8> = (0..32768).map(|x| x as u8).collect(); // 2^15
    let _ = BytesMut::from_vec(vec);
}

#[test]
fn test_from_vec_max_capacity() {
    let vec: Vec<u8> = (0..131071).map(|x| x as u8).collect(); // 2^17 - 1
    let _ = BytesMut::from_vec(vec);
}

#[test]
#[should_panic]
fn test_from_vec_over_max_capacity() {
    let vec: Vec<u8> = (0..131072).map(|x| x as u8).collect(); // 2^17
    let _ = BytesMut::from_vec(vec);
} 

#[test]
fn test_from_vec_min_capacity() {
    let vec: Vec<u8> = (0..10).map(|x| x as u8).collect(); // at least 10
    let _ = BytesMut::from_vec(vec);
}


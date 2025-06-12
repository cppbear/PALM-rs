// Answer 0

#[test]
fn test_put_slice_with_empty_bytes_mut() {
    let mut bytes_mut = BytesMut::new();
    let input: &[u8] = &[1, 2, 3];
    bytes_mut.put_slice(input);
    assert_eq!(bytes_mut.len(), input.len());
}

#[test]
fn test_put_slice_with_non_empty_bytes_mut() {
    let mut bytes_mut = BytesMut::from_vec(vec![4, 5, 6]);
    let input: &[u8] = &[1, 2, 3];
    bytes_mut.put_slice(input);
    assert_eq!(bytes_mut.len(), 6);
    assert_eq!(bytes_mut.as_slice(), &[4, 5, 6, 1, 2, 3]);
}

#[test]
fn test_put_slice_with_large_input() {
    let mut bytes_mut = BytesMut::new();
    let large_input: Vec<u8> = (0..10_000).map(|x| x as u8).collect();
    bytes_mut.put_slice(&large_input);
    assert_eq!(bytes_mut.len(), large_input.len());
}

#[test]
fn test_put_slice_with_repeat_input() {
    let mut bytes_mut = BytesMut::new();
    let input: &[u8] = &[1, 1, 1, 1, 1];
    bytes_mut.put_slice(input);
    assert_eq!(bytes_mut.len(), input.len());
    assert_eq!(bytes_mut.as_slice(), &[1, 1, 1, 1, 1]);
}


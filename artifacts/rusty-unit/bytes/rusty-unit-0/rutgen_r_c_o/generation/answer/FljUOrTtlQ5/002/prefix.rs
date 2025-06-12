// Answer 0

#[test]
fn test_put_with_empty_src() {
    let mut bytes_mut = BytesMut::new();
    let empty_src = vec![].into_iter();
    bytes_mut.put(empty_src);
}

#[test]
fn test_put_with_single_element_src() {
    let mut bytes_mut = BytesMut::new();
    let single_elem_src = vec![42].into_iter();
    bytes_mut.put(single_elem_src);
}

#[test]
fn test_put_with_two_elements_src() {
    let mut bytes_mut = BytesMut::new();
    let two_elems_src = vec![1, 2].into_iter();
    bytes_mut.put(two_elems_src);
}

#[test]
fn test_put_with_large_src() {
    let mut bytes_mut = BytesMut::with_capacity(1024);
    let large_src = (0..1024).collect::<Vec<u8>>().into_iter();
    bytes_mut.put(large_src);
}

#[test]
fn test_put_with_exactly_full_capacity_src() {
    let capacity = 16;
    let mut bytes_mut = BytesMut::with_capacity(capacity);
    let full_capacity_src = (0..capacity).collect::<Vec<u8>>().into_iter();
    bytes_mut.put(full_capacity_src);
}

#[test]
fn test_put_with_multiple_chunks() {
    let mut bytes_mut = BytesMut::new();
    let chunks = vec![vec![1, 2, 3], vec![4, 5], vec![6]].into_iter();
    for chunk in chunks {
        bytes_mut.put(chunk.into_iter());
    }
}


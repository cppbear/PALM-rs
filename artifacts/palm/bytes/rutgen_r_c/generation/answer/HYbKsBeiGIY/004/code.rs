// Answer 0

#[test]
fn test_chunks_vectored_multiple_slices() {
    use std::io::IoSlice;
    use alloc::collections::VecDeque;

    let mut deque: VecDeque<u8> = VecDeque::from(vec![1, 2, 3, 4, 5, 6]);
    let mut dst: [IoSlice; 2] = Default::default(); // Ensure there are two slices available

    let result = deque.chunks_vectored(&mut dst);

    assert_eq!(result, 2);
    assert_eq!(dst[0].len(), 3); // Check the first slice length
    assert_eq!(dst[1].len(), 3); // Check the second slice length
    assert_eq!(dst[0], IoSlice::new(&[1, 2, 3])); // Verify contents of first slice
    assert_eq!(dst[1], IoSlice::new(&[4, 5, 6])); // Verify contents of second slice
}

#[test]
fn test_chunks_vectored_single_slice() {
    use std::io::IoSlice;
    use alloc::collections::VecDeque;

    let mut deque: VecDeque<u8> = VecDeque::from(vec![1, 2, 3]);
    let mut dst: [IoSlice; 1] = Default::default(); // Ensure only one slice is available

    let result = deque.chunks_vectored(&mut dst);

    assert_eq!(result, 1);
    assert_eq!(dst[0].len(), 3); // Check the first slice length
    assert_eq!(dst[0], IoSlice::new(&[1, 2, 3])); // Verify contents of first slice
}

#[test]
fn test_chunks_vectored_empty_input() {
    use std::io::IoSlice;
    use alloc::collections::VecDeque;

    let deque: VecDeque<u8> = VecDeque::new(); // Empty deque
    let mut dst: [IoSlice; 2] = Default::default(); // Destination isn't empty

    let result = deque.chunks_vectored(&mut dst);

    assert_eq!(result, 0); // Check the result from empty deque
}

#[test]
fn test_chunks_vectored_dst_empty() {
    use std::io::IoSlice;
    use alloc::collections::VecDeque;

    let mut deque: VecDeque<u8> = VecDeque::from(vec![1, 2, 3, 4, 5]);
    let mut dst: [IoSlice; 0] = []; // Empty destination

    let result = deque.chunks_vectored(&mut dst);

    assert_eq!(result, 0); // Check the result from empty destination
}


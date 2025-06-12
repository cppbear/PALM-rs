// Answer 0

#[test]
fn test_chunks_vectored_with_single_slice() {
    use std::collections::VecDeque;
    use std::io::IoSlice;

    let mut vec_deque: VecDeque<u8> = VecDeque::from(vec![1, 2, 3, 4]);
    let mut io_slices: [IoSlice; 2] = Default::default();

    let result = vec_deque.chunks_vectored(&mut io_slices);

    assert_eq!(result, 1);
    assert_eq!(io_slices[0].as_ref(), &[1, 2, 3, 4]);
    assert!(io_slices[1].is_empty());
}

#[test]
fn test_chunks_vectored_with_two_slices() {
    use std::collections::VecDeque;
    use std::io::IoSlice;

    let mut vec_deque: VecDeque<u8> = VecDeque::from(vec![1, 2, 3, 4, 5, 6, 7, 8]);
    let mut io_slices: [IoSlice; 2] = Default::default();

    let result = vec_deque.chunks_vectored(&mut io_slices);

    assert_eq!(result, 2);
    assert_eq!(io_slices[0].as_ref(), &[1, 2, 3, 4]);
    assert_eq!(io_slices[1].as_ref(), &[5, 6, 7, 8]);
}

#[test]
fn test_chunks_vectored_with_empty_slices() {
    use std::collections::VecDeque;
    use std::io::IoSlice;

    let mut vec_deque: VecDeque<u8> = VecDeque::new();
    let mut io_slices: [IoSlice; 2] = Default::default();

    let result = vec_deque.chunks_vectored(&mut io_slices);

    assert_eq!(result, 0);
    assert!(io_slices[0].is_empty());
    assert!(io_slices[1].is_empty());
}


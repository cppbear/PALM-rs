// Answer 0

#[test]
fn test_chunks_vectored_non_empty_with_single_slice() {
    let mut deque = VecDeque::from(vec![1u8]);
    let mut dst = [io::IoSlice::new(&[]); 2];
    let result = deque.chunks_vectored(&mut dst);
}

#[test]
fn test_chunks_vectored_non_empty_with_double_slice_second_empty() {
    let mut deque = VecDeque::from(vec![1u8, 2u8]);
    let mut dst = [io::IoSlice::new(&[]); 2];
    let result = deque.chunks_vectored(&mut dst);
}


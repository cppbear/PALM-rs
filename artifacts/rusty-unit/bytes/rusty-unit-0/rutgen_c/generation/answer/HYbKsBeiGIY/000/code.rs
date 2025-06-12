// Answer 0

#[test]
fn test_chunks_vectored_empty_vec_deque() {
    use alloc::collections::VecDeque;
    use std::io;

    let vec_deque: VecDeque<u8> = VecDeque::new();
    let mut slices = [io::IoSlice::new(&[])];
    let result = vec_deque.chunks_vectored(&mut slices);
    assert_eq!(result, 0);
}

#[test]
fn test_chunks_vectored_single_slice() {
    use alloc::collections::VecDeque;
    use std::io;

    let mut vec_deque: VecDeque<u8> = VecDeque::new();
    vec_deque.push_back(1);
    let mut slices = [io::IoSlice::new(&[])];
    let result = vec_deque.chunks_vectored(&mut slices);
    assert_eq!(result, 1);
    assert_eq!(slices[0].as_ref(), [1]);
}

#[test]
fn test_chunks_vectored_two_slices() {
    use alloc::collections::VecDeque;
    use std::io;

    let mut vec_deque: VecDeque<u8> = VecDeque::new();
    vec_deque.push_back(1);
    vec_deque.push_back(2);
    let mut slices = [io::IoSlice::new(&[]), io::IoSlice::new(&[])];
    let result = vec_deque.chunks_vectored(&mut slices);
    assert_eq!(result, 2);
    assert_eq!(slices[0].as_ref(), [1]);
    assert_eq!(slices[1].as_ref(), [2]);
}

#[test]
fn test_chunks_vectored_no_space_in_dst() {
    use alloc::collections::VecDeque;
    use std::io;

    let mut vec_deque: VecDeque<u8> = VecDeque::new();
    vec_deque.push_back(1);
    vec_deque.push_back(2);
    let mut slices = [io::IoSlice::new(&[])];
    let result = vec_deque.chunks_vectored(&mut slices);
    assert_eq!(result, 1);
    assert_eq!(slices[0].as_ref(), [1]);
}


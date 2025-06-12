// Answer 0

#[test]
fn test_chunk_returns_first_slice_when_not_empty() {
    let mut deque = VecDeque::from(vec![1, 2, 3]);
    let chunk = deque.chunk();
    assert_eq!(chunk, &[1, 2, 3]);
}

#[test]
fn test_chunk_returns_second_slice_when_first_empty() {
    let mut deque = VecDeque::from(vec![]);
    deque.push_back(4);
    let chunk = deque.chunk();
    assert_eq!(chunk, &[4]);
}

#[test]
fn test_chunk_empty_vec_deque() {
    let deque: VecDeque<u8> = VecDeque::new();
    let chunk = deque.chunk();
    assert_eq!(chunk, &[]);
}


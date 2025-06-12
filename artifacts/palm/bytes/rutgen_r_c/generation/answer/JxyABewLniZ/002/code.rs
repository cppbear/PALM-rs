// Answer 0

#[test]
fn test_chunk_non_empty_s1() {
    let mut deque = VecDeque::from(vec![1, 2, 3, 4]);
    let chunk = deque.chunk();
    assert_eq!(chunk, &[1, 2, 3, 4]);
}

#[test]
fn test_chunk_non_empty_s1_after_multiple_pushes() {
    let mut deque = VecDeque::new();
    deque.push_back(5);
    deque.push_back(6);
    let chunk = deque.chunk();
    assert_eq!(chunk, &[5, 6]);
}

#[test]
fn test_chunk_non_empty_s1_after_advancing() {
    let mut deque = VecDeque::from(vec![10, 20, 30]);
    deque.advance(1);
    let chunk = deque.chunk();
    assert_eq!(chunk, &[20, 30]);
}


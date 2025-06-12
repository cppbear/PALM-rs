// Answer 0

#[test]
fn test_advance_empty_vec_deque() {
    let mut deque: VecDeque<u8> = VecDeque::new();
    deque.advance(0);
    assert_eq!(deque.len(), 0);
}

#[test]
fn test_advance_non_empty_vec_deque() {
    let mut deque: VecDeque<u8> = VecDeque::from(vec![1, 2, 3, 4, 5]);
    deque.advance(2);
    assert_eq!(deque.len(), 3);
    assert_eq!(deque[0], 3);
    assert_eq!(deque[1], 4);
    assert_eq!(deque[2], 5);
}

#[test]
fn test_advance_full_vec_deque() {
    let mut deque: VecDeque<u8> = VecDeque::from(vec![1, 2, 3]);
    deque.advance(3);
    assert_eq!(deque.len(), 0);
}

#[should_panic]
fn test_advance_too_many_elements() {
    let mut deque: VecDeque<u8> = VecDeque::from(vec![1, 2, 3]);
    deque.advance(4);
}


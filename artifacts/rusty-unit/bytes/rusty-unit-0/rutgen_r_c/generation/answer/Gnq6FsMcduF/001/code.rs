// Answer 0

#[test]
fn test_advance_non_panic() {
    let mut deque: VecDeque<u8> = VecDeque::from(vec![1, 2, 3, 4, 5]);
    deque.advance(3);
    assert_eq!(deque.len(), 2);
    assert_eq!(deque.front(), Some(&4));
}

#[test]
fn test_advance_to_empty() {
    let mut deque: VecDeque<u8> = VecDeque::from(vec![1, 2, 3]);
    deque.advance(3);
    assert_eq!(deque.len(), 0);
}

#[should_panic]
fn test_advance_panic_out_of_bounds() {
    let mut deque: VecDeque<u8> = VecDeque::from(vec![1, 2]);
    deque.advance(3);
}


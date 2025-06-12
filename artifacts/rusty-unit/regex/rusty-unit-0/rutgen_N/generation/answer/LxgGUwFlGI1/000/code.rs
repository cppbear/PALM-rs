// Answer 0

#[derive(Debug)]
struct Input {
    pos: usize,
    len: usize,
}

impl Input {
    pub fn new(pos: usize, len: usize) -> Self {
        Input { pos, len }
    }
    
    pub fn next_pos(&self) -> usize {
        self.pos + self.len
    }
}

#[test]
fn test_next_pos_basic() {
    let input = Input::new(5, 3);
    assert_eq!(input.next_pos(), 8);
}

#[test]
fn test_next_pos_zero_length() {
    let input = Input::new(10, 0);
    assert_eq!(input.next_pos(), 10);
}

#[test]
fn test_next_pos_large_values() {
    let input = Input::new(usize::MAX - 1, 1);
    assert_eq!(input.next_pos(), usize::MAX);
}

#[test]
fn test_next_pos_edge_case() {
    let input = Input::new(0, 0);
    assert_eq!(input.next_pos(), 0);
}


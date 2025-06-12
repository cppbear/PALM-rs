// Answer 0

#[derive(Debug)]
struct Input {
    pos: usize,
}

impl Input {
    pub fn new(pos: usize) -> Self {
        Input { pos }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }
}

#[test]
fn test_pos_zero() {
    let input = Input::new(0);
    assert_eq!(input.pos(), 0);
}

#[test]
fn test_pos_positive() {
    let input = Input::new(10);
    assert_eq!(input.pos(), 10);
}

#[test]
fn test_pos_large() {
    let input = Input::new(usize::MAX);
    assert_eq!(input.pos(), usize::MAX);
}


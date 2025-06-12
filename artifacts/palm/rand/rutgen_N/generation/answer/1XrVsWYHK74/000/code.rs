// Answer 0

#[derive(Default)]
struct Buffer {
    index: usize,
}

impl Buffer {
    pub fn index(&self) -> usize {
        self.index
    }
}

#[test]
fn test_index_zero() {
    let buffer = Buffer::default();
    assert_eq!(buffer.index(), 0);
}

#[test]
fn test_index_non_zero() {
    let mut buffer = Buffer::default();
    buffer.index = 5;
    assert_eq!(buffer.index(), 5);
}

#[test]
fn test_index_boundary() {
    let mut buffer = Buffer::default();
    buffer.index = usize::MAX;
    assert_eq!(buffer.index(), usize::MAX);
}

#[test]
fn test_index_after_reset() {
    let mut buffer = Buffer::default();
    buffer.index = 10;
    buffer.index = 0; // Simulating a 'reset' for the boundary condition
    assert_eq!(buffer.index(), 0);
}


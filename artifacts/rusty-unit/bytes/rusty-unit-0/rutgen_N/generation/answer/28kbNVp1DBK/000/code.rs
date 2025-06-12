// Answer 0

#[derive(Default)]
struct Buffer {
    data: Vec<u8>,
    position: usize,
}

impl Buffer {
    fn remaining_mut(&self) -> usize {
        self.data.len() - self.position
    }
}

struct BufferWrapper<'a> {
    buffer: &'a mut Buffer,
}

impl<'a> BufferWrapper<'a> {
    fn remaining_mut(&self) -> usize {
        self.buffer.remaining_mut()
    }
}

#[test]
fn test_remaining_mut() {
    let mut buffer = Buffer {
        data: vec![1, 2, 3, 4, 5],
        position: 2,
    };
    let wrapper = BufferWrapper { buffer: &mut buffer };
    assert_eq!(wrapper.remaining_mut(), 3);
}

#[test]
fn test_remaining_mut_empty_buffer() {
    let mut buffer = Buffer {
        data: Vec::new(),
        position: 0,
    };
    let wrapper = BufferWrapper { buffer: &mut buffer };
    assert_eq!(wrapper.remaining_mut(), 0);
}

#[test]
fn test_remaining_mut_at_end() {
    let mut buffer = Buffer {
        data: vec![1, 2, 3],
        position: 3,
    };
    let wrapper = BufferWrapper { buffer: &mut buffer };
    assert_eq!(wrapper.remaining_mut(), 0);
}


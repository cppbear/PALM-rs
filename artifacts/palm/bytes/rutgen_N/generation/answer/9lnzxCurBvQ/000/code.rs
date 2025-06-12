// Answer 0

#[derive(Debug)]
struct MockBuffer {
    remaining: usize,
    data: Vec<u8>,
}

impl MockBuffer {
    fn new(remaining: usize, data: Vec<u8>) -> Self {
        Self { remaining, data }
    }

    fn has_remaining_mut(&self) -> bool {
        self.remaining > 0
    }

    fn chunk_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }
}

struct Chain<'a> {
    a: &'a mut MockBuffer,
    b: &'a mut MockBuffer,
}

impl<'a> Chain<'a> {
    fn new(a: &'a mut MockBuffer, b: &'a mut MockBuffer) -> Self {
        Self { a, b }
    }

    fn chunk_mut(&mut self) -> &mut [u8] {
        if self.a.has_remaining_mut() {
            self.a.chunk_mut()
        } else {
            self.b.chunk_mut()
        }
    }
}

#[test]
fn test_chunk_mut_a_has_remaining() {
    let mut buffer_a = MockBuffer::new(1, vec![1, 2, 3]);
    let mut buffer_b = MockBuffer::new(0, vec![4, 5, 6]);
    let mut chain = Chain::new(&mut buffer_a, &mut buffer_b);
    
    let chunk = chain.chunk_mut();
    assert_eq!(chunk.len(), 3);
    assert_eq!(chunk[0], 1);
}

#[test]
fn test_chunk_mut_b_has_remaining() {
    let mut buffer_a = MockBuffer::new(0, vec![1, 2, 3]);
    let mut buffer_b = MockBuffer::new(1, vec![4, 5, 6]);
    let mut chain = Chain::new(&mut buffer_a, &mut buffer_b);
    
    let chunk = chain.chunk_mut();
    assert_eq!(chunk.len(), 3);
    assert_eq!(chunk[0], 4);
}

#[test]
fn test_chunk_mut_b_empty() {
    let mut buffer_a = MockBuffer::new(0, vec![1, 2, 3]);
    let mut buffer_b = MockBuffer::new(0, vec![]);
    let mut chain = Chain::new(&mut buffer_a, &mut buffer_b);
    
    let chunk = chain.chunk_mut();
    assert_eq!(chunk.len(), 0);
}


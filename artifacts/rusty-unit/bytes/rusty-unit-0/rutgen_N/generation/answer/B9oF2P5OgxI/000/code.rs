// Answer 0

#[derive(Default)]
struct Buffer {
    data: Vec<u8>,
}

impl Buffer {
    fn remaining(&self) -> usize {
        self.data.len()
    }
}

struct Chain {
    a: Buffer,
    b: Buffer,
}

impl Chain {
    fn remaining(&self) -> usize {
        self.a.remaining().saturating_add(self.b.remaining())
    }
}

#[test]
fn test_chain_remaining_both_buffers_empty() {
    let chain = Chain {
        a: Buffer::default(),
        b: Buffer::default(),
    };
    assert_eq!(chain.remaining(), 0);
}

#[test]
fn test_chain_remaining_first_buffer_non_empty() {
    let chain = Chain {
        a: Buffer { data: vec![1, 2, 3] },
        b: Buffer::default(),
    };
    assert_eq!(chain.remaining(), 3);
}

#[test]
fn test_chain_remaining_second_buffer_non_empty() {
    let chain = Chain {
        a: Buffer::default(),
        b: Buffer { data: vec![4, 5] },
    };
    assert_eq!(chain.remaining(), 2);
}

#[test]
fn test_chain_remaining_both_buffers_non_empty() {
    let chain = Chain {
        a: Buffer { data: vec![1, 2] },
        b: Buffer { data: vec![3, 4, 5] },
    };
    assert_eq!(chain.remaining(), 5);
}


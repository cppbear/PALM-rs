// Answer 0

#[derive(Default)]
struct InnerBuffer {
    a: usize,
}

impl InnerBuffer {
    fn remaining_mut(&self) -> usize {
        self.a
    }
}

struct BufferChain {
    a: InnerBuffer,
    b: InnerBuffer,
}

impl BufferChain {
    fn remaining_mut(&self) -> usize {
        self.a
            .remaining_mut()
            .saturating_add(self.b.remaining_mut())
    }
}

#[test]
fn test_remaining_mut_both_zero() {
    let buffer_chain = BufferChain {
        a: InnerBuffer { a: 0 },
        b: InnerBuffer { a: 0 },
    };
    assert_eq!(buffer_chain.remaining_mut(), 0);
}

#[test]
fn test_remaining_mut_a_zero_b_nonzero() {
    let buffer_chain = BufferChain {
        a: InnerBuffer { a: 0 },
        b: InnerBuffer { a: 5 },
    };
    assert_eq!(buffer_chain.remaining_mut(), 5);
}

#[test]
fn test_remaining_mut_a_nonzero_b_zero() {
    let buffer_chain = BufferChain {
        a: InnerBuffer { a: 3 },
        b: InnerBuffer { a: 0 },
    };
    assert_eq!(buffer_chain.remaining_mut(), 3);
}

#[test]
fn test_remaining_mut_a_nonzero_b_nonzero() {
    let buffer_chain = BufferChain {
        a: InnerBuffer { a: 4 },
        b: InnerBuffer { a: 6 },
    };
    assert_eq!(buffer_chain.remaining_mut(), 10);
}


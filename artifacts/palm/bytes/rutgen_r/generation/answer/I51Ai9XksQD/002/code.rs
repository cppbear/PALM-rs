// Answer 0

#[derive(Default)]
struct A {
    remaining: usize,
}

impl A {
    fn has_remaining(&self) -> bool {
        self.remaining > 0
    }
    
    fn chunk(&self) -> &[u8] {
        &[]
    }
}

#[derive(Default)]
struct B {
    data: Vec<u8>,
}

impl B {
    fn chunk(&self) -> &[u8] {
        &self.data
    }
}

struct Chain {
    a: A,
    b: B,
}

#[test]
fn test_chunk_b_return() {
    let a = A { remaining: 0 }; // self.a.has_remaining() is false
    let b = B { data: vec![1, 2, 3] }; // some sample data
    
    let chain = Chain { a, b };
    
    assert_eq!(chain.chunk(), &[1, 2, 3]);
}

#[test]
fn test_chunk_b_empty_return() {
    let a = A { remaining: 0 }; // self.a.has_remaining() is false
    let b = B { data: vec![] }; // empty data
    
    let chain = Chain { a, b };
    
    assert_eq!(chain.chunk(), &[]);
}


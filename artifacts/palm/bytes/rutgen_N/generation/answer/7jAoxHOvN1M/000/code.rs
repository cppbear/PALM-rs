// Answer 0

#[derive(Default)]
struct BufMutStruct {
    limit: usize,
}

impl BufMutStruct {
    pub fn new(limit: usize) -> Self {
        BufMutStruct { limit }
    }

    pub fn limit(&self) -> usize {
        self.limit
    }
}

#[test]
fn test_limit() {
    let buf = BufMutStruct::new(1024);
    assert_eq!(buf.limit(), 1024);
}

#[test]
fn test_limit_zero() {
    let buf = BufMutStruct::new(0);
    assert_eq!(buf.limit(), 0);
}

#[test]
fn test_limit_large_value() {
    let buf = BufMutStruct::new(usize::MAX);
    assert_eq!(buf.limit(), usize::MAX);
}


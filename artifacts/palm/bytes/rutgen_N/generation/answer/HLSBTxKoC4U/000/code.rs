// Answer 0

#[derive(Debug)]
struct BufMutMock {
    position: usize,
    data: Vec<u8>,
}

impl BufMutMock {
    fn new(data: Vec<u8>) -> Self {
        BufMutMock { position: 0, data }
    }

    unsafe fn advance_mut(&mut self, cnt: usize) {
        self.position += cnt.min(self.data.len() - self.position);
    }
}

#[test]
fn test_advance_mut_within_bounds() {
    let mut buf = BufMutMock::new(vec![1, 2, 3, 4, 5]);
    unsafe {
        buf.advance_mut(2);
    }
    assert_eq!(buf.position, 2);
}

#[test]
fn test_advance_mut_exact_bounds() {
    let mut buf = BufMutMock::new(vec![1, 2, 3, 4, 5]);
    unsafe {
        buf.advance_mut(5);
    }
    assert_eq!(buf.position, 5);
}

#[test]
fn test_advance_mut_exceeding_bounds() {
    let mut buf = BufMutMock::new(vec![1, 2, 3, 4, 5]);
    unsafe {
        buf.advance_mut(10);
    }
    assert_eq!(buf.position, 5);
}

#[test]
fn test_advance_mut_zero() {
    let mut buf = BufMutMock::new(vec![1, 2, 3, 4, 5]);
    unsafe {
        buf.advance_mut(0);
    }
    assert_eq!(buf.position, 0);
}


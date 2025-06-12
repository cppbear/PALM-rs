// Answer 0

#[repr(C)]
struct Inner {
    data: Vec<u8>,
    index: usize,
}

impl Inner {
    fn advance_mut(&mut self, cnt: usize) {
        self.index += cnt;
    }
}

struct Limit {
    inner: Inner,
    limit: usize,
}

impl Limit {
    fn new(data: Vec<u8>, limit: usize) -> Self {
        Self {
            inner: Inner { data, index: 0 },
            limit,
        }
    }

    unsafe fn advance_mut(&mut self, cnt: usize) {
        assert!(cnt <= self.limit);
        self.inner.advance_mut(cnt);
        self.limit -= cnt;
    }
}

#[test]
fn test_advance_mut_within_limit() {
    let mut limit = Limit::new(vec![0, 1, 2, 3, 4], 5);
    unsafe {
        limit.advance_mut(3);
    }
    assert_eq!(limit.limit, 2);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_advance_mut_exceeding_limit() {
    let mut limit = Limit::new(vec![0, 1, 2, 3, 4], 2);
    unsafe {
        limit.advance_mut(3);
    }
}

#[test]
fn test_advance_mut_exact_limit() {
    let mut limit = Limit::new(vec![0, 1, 2, 3, 4], 5);
    unsafe {
        limit.advance_mut(5);
    }
    assert_eq!(limit.limit, 0);
}


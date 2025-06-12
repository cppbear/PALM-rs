// Answer 0

#[derive(Default)]
struct Inner {
    position: usize,
}

impl Inner {
    fn advance(&mut self, cnt: usize) {
        self.position += cnt;
    }
}

struct Take {
    inner: Inner,
    limit: usize,
}

impl Take {
    fn new(inner: Inner, limit: usize) -> Self {
        Take { inner, limit }
    }

    fn advance(&mut self, cnt: usize) {
        assert!(cnt <= self.limit);
        self.inner.advance(cnt);
        self.limit -= cnt;
    }
}

#[test]
fn test_advance_within_limit() {
    let mut inner = Inner::default();
    let mut take = Take::new(inner, 5);
    take.advance(3);
    assert_eq!(take.limit, 2);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn test_advance_exceeding_limit() {
    let mut inner = Inner::default();
    let mut take = Take::new(inner, 2);
    take.advance(3);
}

#[test]
fn test_advance_to_zero_limit() {
    let mut inner = Inner::default();
    let mut take = Take::new(inner, 4);
    take.advance(4);
    assert_eq!(take.limit, 0);
}


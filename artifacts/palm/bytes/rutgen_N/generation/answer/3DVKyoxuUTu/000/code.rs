// Answer 0

#[derive(Debug)]
struct Bytes {
    data: Vec<u8>,
    start: usize,
}

impl Bytes {
    fn new(data: Vec<u8>) -> Self {
        Self { data, start: 0 }
    }

    fn len(&self) -> usize {
        self.data.len() - self.start
    }

    unsafe fn inc_start(&mut self, cnt: usize) {
        self.start += cnt;
    }

    fn advance(&mut self, cnt: usize) {
        assert!(
            cnt <= self.len(),
            "cannot advance past `remaining`: {:?} <= {:?}",
            cnt,
            self.len(),
        );

        unsafe {
            self.inc_start(cnt);
        }
    }
}

#[test]
fn test_advance_within_bounds() {
    let mut bytes = Bytes::new(vec![1, 2, 3, 4, 5]);
    bytes.advance(2);
    assert_eq!(bytes.len(), 3);
}

#[test]
#[should_panic(expected = "cannot advance past `remaining`")]
fn test_advance_past_bounds() {
    let mut bytes = Bytes::new(vec![1, 2, 3]);
    bytes.advance(4);
}


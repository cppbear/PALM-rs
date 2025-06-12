// Answer 0

#[derive(Default)]
struct Bytes {
    data: Vec<u8>,
    start: usize,
}

impl Bytes {
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
#[should_panic(expected = "cannot advance past `remaining`: 5 <= 3")]
fn test_advance_panic() {
    let mut bytes = Bytes {
        data: vec![1, 2, 3],
        start: 0,
    };
    bytes.advance(5); // This should panic since 5 > len (3)
}

#[test]
fn test_advance_no_panic() {
    let mut bytes = Bytes {
        data: vec![1, 2, 3],
        start: 0,
    };
    bytes.advance(3); // This should work since 3 == len (3)
    assert_eq!(bytes.len(), 0); // After advancing all, len should be 0
}

#[test]
fn test_advance_edge_case() {
    let mut bytes = Bytes {
        data: vec![1, 2],
        start: 0,
    };
    bytes.advance(2); // Advance to the end
    assert_eq!(bytes.len(), 0); // Verify length is now 0
}


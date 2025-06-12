// Answer 0

#[test]
fn test_advance_within_bounds() {
    struct BytesMut {
        data: Vec<u8>,
        position: usize,
    }

    impl BytesMut {
        fn new(size: usize) -> Self {
            Self {
                data: vec![0; size],
                position: 0,
            }
        }

        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        unsafe fn advance_unchecked(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn advance(&mut self, cnt: usize) {
            assert!(
                cnt <= self.remaining(),
                "cannot advance past `remaining`: {:?} <= {:?}",
                cnt,
                self.remaining(),
            );
            unsafe {
                self.advance_unchecked(cnt);
            }
        }
    }

    let mut bytes = BytesMut::new(10);
    bytes.advance(5);
    assert_eq!(bytes.position, 5);
}

#[test]
#[should_panic(expected = "cannot advance past `remaining`: 6 <= 5")]
fn test_advance_exceeding_bounds() {
    struct BytesMut {
        data: Vec<u8>,
        position: usize,
    }

    impl BytesMut {
        fn new(size: usize) -> Self {
            Self {
                data: vec![0; size],
                position: 0,
            }
        }

        fn remaining(&self) -> usize {
            self.data.len() - self.position
        }

        unsafe fn advance_unchecked(&mut self, cnt: usize) {
            self.position += cnt;
        }

        fn advance(&mut self, cnt: usize) {
            assert!(
                cnt <= self.remaining(),
                "cannot advance past `remaining`: {:?} <= {:?}",
                cnt,
                self.remaining(),
            );
            unsafe {
                self.advance_unchecked(cnt);
            }
        }
    }

    let mut bytes = BytesMut::new(5);
    bytes.advance(6);
}


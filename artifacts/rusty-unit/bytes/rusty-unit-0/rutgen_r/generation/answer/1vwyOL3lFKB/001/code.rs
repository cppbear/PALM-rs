// Answer 0

#[test]
fn test_advance_exact_remaining() {
    struct BytesMut {
        data: Vec<u8>,
        position: usize,
    }

    impl BytesMut {
        fn new(capacity: usize) -> Self {
            BytesMut {
                data: vec![0; capacity],
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
    let cnt = bytes.remaining();
    bytes.advance(cnt);
    assert_eq!(bytes.remaining(), 0);
}

#[test]
#[should_panic(expected = "cannot advance past `remaining`: 1 <= 0")]
fn test_advance_panic_exceeding_remaining() {
    struct BytesMut {
        data: Vec<u8>,
        position: usize,
    }

    impl BytesMut {
        fn new(capacity: usize) -> Self {
            BytesMut {
                data: vec![0; capacity],
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
    bytes.advance(1); // simulate some advances
    assert_eq!(bytes.remaining(), 10);
    bytes.advance(11); // should panic here
}


// Answer 0

#[test]
fn test_advance_boundary_case() {
    struct TestBuf {
        data: Vec<u8>,
        position: u64,
    }

    impl TestBuf {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }

        fn get_ref(&self) -> &[u8] {
            &self.data
        }

        fn position(&self) -> u64 {
            self.position
        }

        fn set_position(&mut self, pos: u64) {
            self.position = pos;
        }
    }

    fn saturating_sub_usize_u64(len: usize, pos: u64) -> usize {
        if pos as usize >= len {
            0
        } else {
            len - pos as usize
        }
    }

    fn panic_advance(error: &TryGetError) {
        panic!("Panic triggered: requested = {}, available = {}", error.requested, error.available);
    }

    struct TryGetError {
        requested: usize,
        available: usize,
    }

    let mut buf = TestBuf::new(vec![0u8; 10]); // Buffer of length 10
    buf.set_position(5); // Current position is 5

    let len = buf.get_ref().len();
    let pos = buf.position();
    let max_cnt = saturating_sub_usize_u64(len, pos);

    // Set cnt to max_cnt
    let cnt = max_cnt;

    // Advance without panic
    buf.advance(cnt); 

    assert_eq!(buf.position(), 5 + cnt as u64); // Position should now be 10
}

#[test]
#[should_panic]
fn test_advance_exceeding_cnt() {
    struct TestBuf {
        data: Vec<u8>,
        position: u64,
    }

    impl TestBuf {
        fn new(data: Vec<u8>) -> Self {
            Self { data, position: 0 }
        }

        fn get_ref(&self) -> &[u8] {
            &self.data
        }

        fn position(&self) -> u64 {
            self.position
        }

        fn set_position(&mut self, pos: u64) {
            self.position = pos;
        }
    }

    fn saturating_sub_usize_u64(len: usize, pos: u64) -> usize {
        if pos as usize >= len {
            0
        } else {
            len - pos as usize
        }
    }

    fn panic_advance(error: &TryGetError) {
        panic!("Panic triggered: requested = {}, available = {}", error.requested, error.available);
    }

    struct TryGetError {
        requested: usize,
        available: usize,
    }

    let mut buf = TestBuf::new(vec![0u8; 10]); // Buffer of length 10
    buf.set_position(5); // Current position is 5

    let len = buf.get_ref().len();
    let pos = buf.position();
    let max_cnt = saturating_sub_usize_u64(len, pos);

    // Set cnt to max_cnt + 1, which should trigger a panic
    let cnt = max_cnt + 1;

    // Advance should panic
    buf.advance(cnt); 
}


// Answer 0

#[test]
fn test_remaining_empty_buffer() {
    struct TestBuf {
        data: Vec<u8>,
        pos: usize,
    }

    impl TestBuf {
        fn get_ref(&self) -> &[u8] {
            &self.data
        }
        
        fn position(&self) -> usize {
            self.pos
        }
        
        fn remaining(&self) -> usize {
            saturating_sub_usize_u64(self.get_ref().len(), self.position())
        }
    }

    fn saturating_sub_usize_u64(a: usize, b: usize) -> usize {
        a.saturating_sub(b)
    }

    let buf = TestBuf {
        data: Vec::new(),
        pos: 0,
    };
    
    assert_eq!(buf.remaining(), 0);
}

#[test]
fn test_remaining_non_empty_buffer() {
    struct TestBuf {
        data: Vec<u8>,
        pos: usize,
    }

    impl TestBuf {
        fn get_ref(&self) -> &[u8] {
            &self.data
        }

        fn position(&self) -> usize {
            self.pos
        }

        fn remaining(&self) -> usize {
            saturating_sub_usize_u64(self.get_ref().len(), self.position())
        }
    }

    fn saturating_sub_usize_u64(a: usize, b: usize) -> usize {
        a.saturating_sub(b)
    }

    let buf = TestBuf {
        data: vec![1, 2, 3, 4, 5],
        pos: 2,
    };
    
    assert_eq!(buf.remaining(), 3);
}

#[test]
fn test_remaining_beyond_position() {
    struct TestBuf {
        data: Vec<u8>,
        pos: usize,
    }

    impl TestBuf {
        fn get_ref(&self) -> &[u8] {
            &self.data
        }

        fn position(&self) -> usize {
            self.pos
        }

        fn remaining(&self) -> usize {
            saturating_sub_usize_u64(self.get_ref().len(), self.position())
        }
    }

    fn saturating_sub_usize_u64(a: usize, b: usize) -> usize {
        a.saturating_sub(b)
    }

    let buf = TestBuf {
        data: vec![1, 2, 3],
        pos: 5,
    };

    assert_eq!(buf.remaining(), 0);
}


// Answer 0

#[test]
fn test_remaining_with_zero_position() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(data: Vec<u8>, position: usize) -> Self {
            Self { data, position }
        }

        fn remaining(&self) -> usize {
            saturating_sub_usize_u64(self.data.len(), self.position as u64)
        }

        fn get_ref(&self) -> &Vec<u8> {
            &self.data
        }

        fn position(&self) -> usize {
            self.position
        }
    }

    let buf = TestBuf::new(vec![1, 2, 3, 4, 5], 0);
    assert_eq!(buf.remaining(), 5);
}

#[test]
fn test_remaining_with_non_zero_position() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(data: Vec<u8>, position: usize) -> Self {
            Self { data, position }
        }

        fn remaining(&self) -> usize {
            saturating_sub_usize_u64(self.data.len(), self.position as u64)
        }

        fn get_ref(&self) -> &Vec<u8> {
            &self.data
        }

        fn position(&self) -> usize {
            self.position
        }
    }

    let buf = TestBuf::new(vec![1, 2, 3, 4, 5], 3);
    assert_eq!(buf.remaining(), 2);
}

#[test]
fn test_remaining_with_position_equals_length() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(data: Vec<u8>, position: usize) -> Self {
            Self { data, position }
        }

        fn remaining(&self) -> usize {
            saturating_sub_usize_u64(self.data.len(), self.position as u64)
        }

        fn get_ref(&self) -> &Vec<u8> {
            &self.data
        }

        fn position(&self) -> usize {
            self.position
        }
    }

    let buf = TestBuf::new(vec![1, 2, 3, 4, 5], 5);
    assert_eq!(buf.remaining(), 0);
}

#[test]
fn test_remaining_with_position_greater_than_length() {
    struct TestBuf {
        data: Vec<u8>,
        position: usize,
    }

    impl TestBuf {
        fn new(data: Vec<u8>, position: usize) -> Self {
            Self { data, position }
        }

        fn remaining(&self) -> usize {
            saturating_sub_usize_u64(self.data.len(), self.position as u64)
        }

        fn get_ref(&self) -> &Vec<u8> {
            &self.data
        }

        fn position(&self) -> usize {
            self.position
        }
    }

    let buf = TestBuf::new(vec![1, 2, 3, 4, 5], 10);
    assert_eq!(buf.remaining(), 0);
}


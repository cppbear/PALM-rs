// Answer 0

#[test]
fn test_remaining_with_zero_length() {
    struct CursorMock {
        position: u64,
        data: Vec<u8>,
    }

    impl CursorMock {
        fn new(data: Vec<u8>) -> Self {
            Self { position: 0, data }
        }

        fn get_ref(&self) -> &Vec<u8> {
            &self.data
        }

        fn position(&self) -> u64 {
            self.position
        }

        fn remaining(&self) -> usize {
            saturating_sub_usize_u64(self.get_ref().as_ref().len(), self.position())
        }
    }

    let cursor = CursorMock::new(vec![]);
    assert_eq!(cursor.remaining(), 0);
}

#[test]
fn test_remaining_with_varied_lengths() {
    struct CursorMock {
        position: u64,
        data: Vec<u8>,
    }

    impl CursorMock {
        fn new(data: Vec<u8>, position: u64) -> Self {
            Self { position, data }
        }

        fn get_ref(&self) -> &Vec<u8> {
            &self.data
        }

        fn position(&self) -> u64 {
            self.position
        }

        fn remaining(&self) -> usize {
            saturating_sub_usize_u64(self.get_ref().as_ref().len(), self.position())
        }
    }

    let cursor = CursorMock::new(vec![1, 2, 3, 4, 5], 2);
    assert_eq!(cursor.remaining(), 3);

    let cursor = CursorMock::new(vec![1, 2, 3], 3);
    assert_eq!(cursor.remaining(), 0);

    let cursor = CursorMock::new(vec![1, 2], 5);
    assert_eq!(cursor.remaining(), 0);
}

#[test]
fn test_remaining_with_large_position() {
    struct CursorMock {
        position: u64,
        data: Vec<u8>,
    }

    impl CursorMock {
        fn new(data: Vec<u8>, position: u64) -> Self {
            Self { position, data }
        }

        fn get_ref(&self) -> &Vec<u8> {
            &self.data
        }

        fn position(&self) -> u64 {
            self.position
        }

        fn remaining(&self) -> usize {
            saturating_sub_usize_u64(self.get_ref().as_ref().len(), self.position())
        }
    }

    let cursor = CursorMock::new(vec![1, 2, 3], u64::MAX);
    assert_eq!(cursor.remaining(), 0);
}


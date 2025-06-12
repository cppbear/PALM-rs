// Answer 0

#[test]
fn test_truncate_reduces_length_when_len_is_smaller() {
    struct TestMap {
        entries: Vec<i32>,
    }

    impl TestMap {
        fn new() -> Self {
            Self { entries: vec![1, 2, 3, 4, 5] }
        }

        fn len(&self) -> usize {
            self.entries.len()
        }

        fn erase_indices(&mut self, start: usize, end: usize) {
            self.entries.splice(start..end, std::iter::empty());
        }

        fn truncate(&mut self, len: usize) {
            if len < self.len() {
                self.erase_indices(len, self.entries.len());
                self.entries.truncate(len);
            }
        }
    }

    let mut map = TestMap::new();
    map.truncate(3);
    assert_eq!(map.len(), 3);
}

#[test]
fn test_truncate_keeps_length_when_len_is_equal() {
    struct TestMap {
        entries: Vec<i32>,
    }

    impl TestMap {
        fn new() -> Self {
            Self { entries: vec![1, 2, 3, 4, 5] }
        }

        fn len(&self) -> usize {
            self.entries.len()
        }

        fn erase_indices(&mut self, start: usize, end: usize) {
            self.entries.splice(start..end, std::iter::empty());
        }

        fn truncate(&mut self, len: usize) {
            if len < self.len() {
                self.erase_indices(len, self.entries.len());
                self.entries.truncate(len);
            }
        }
    }

    let mut map = TestMap::new();
    map.truncate(5);
    assert_eq!(map.len(), 5);
}

#[test]
fn test_truncate_does_nothing_when_len_is_greater() {
    struct TestMap {
        entries: Vec<i32>,
    }

    impl TestMap {
        fn new() -> Self {
            Self { entries: vec![1, 2, 3, 4, 5] }
        }

        fn len(&self) -> usize {
            self.entries.len()
        }

        fn erase_indices(&mut self, start: usize, end: usize) {
            self.entries.splice(start..end, std::iter::empty());
        }

        fn truncate(&mut self, len: usize) {
            if len < self.len() {
                self.erase_indices(len, self.entries.len());
                self.entries.truncate(len);
            }
        }
    }

    let mut map = TestMap::new();
    map.truncate(6);
    assert_eq!(map.len(), 5);
}


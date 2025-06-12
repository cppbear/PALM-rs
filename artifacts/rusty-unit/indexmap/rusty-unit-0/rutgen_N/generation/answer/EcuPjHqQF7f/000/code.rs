// Answer 0

#[test]
fn test_try_reserve_increases_capacity() {
    struct TestMap {
        entries: Vec<usize>,
        indices: Vec<usize>,
    }

    struct TryReserveError;

    impl TestMap {
        fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
            self.indices.try_reserve(additional)?;
            if additional > self.entries.capacity() - self.entries.len() {
                self.try_reserve_entries(additional)
            } else {
                Ok(())
            }
        }

        fn try_reserve_entries(&mut self, additional: usize) -> Result<(), TryReserveError> {
            self.entries.resize(self.entries.len() + additional, 0);
            Ok(())
        }
    }

    impl Vec<usize> {
        fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
            self.reserve(additional);
            Ok(())
        }
    }

    let mut my_map = TestMap {
        entries: Vec::with_capacity(2),
        indices: Vec::new(),
    };

    assert_eq!(my_map.entries.len(), 0);
    assert!(my_map.try_reserve(3).is_ok());
    assert_eq!(my_map.entries.capacity(), 3);
}

#[test]
fn test_try_reserve_no_growth_needed() {
    struct TestMap {
        entries: Vec<usize>,
        indices: Vec<usize>,
    }

    struct TryReserveError;

    impl TestMap {
        fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
            self.indices.try_reserve(additional)?;
            if additional > self.entries.capacity() - self.entries.len() {
                self.try_reserve_entries(additional)
            } else {
                Ok(())
            }
        }

        fn try_reserve_entries(&mut self, _: usize) -> Result<(), TryReserveError> {
            Ok(())
        }
    }

    impl Vec<usize> {
        fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
            self.reserve(additional);
            Ok(())
        }
    }

    let mut my_map = TestMap {
        entries: Vec::with_capacity(5),
        indices: Vec::new(),
    };

    my_map.entries.push(1);
    assert_eq!(my_map.entries.len(), 1);
    assert!(my_map.try_reserve(4).is_ok());
    assert_eq!(my_map.entries.capacity(), 5);
}

#[test]
#[should_panic]
fn test_try_reserve_exceeding_capacity() {
    struct TestMap {
        entries: Vec<usize>,
        indices: Vec<usize>,
    }

    struct TryReserveError;

    impl TestMap {
        fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
            self.indices.try_reserve(additional)?;
            if additional > self.entries.capacity() - self.entries.len() {
                self.try_reserve_entries(additional)
            } else {
                Ok(())
            }
        }

        fn try_reserve_entries(&mut self, additional: usize) -> Result<(), TryReserveError> {
            self.entries.resize(self.entries.len() + additional, 0);
            assert!(self.entries.len() <= self.entries.capacity());
            Ok(())
        }
    }

    impl Vec<usize> {
        fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
            self.reserve(additional);
            Ok(())
        }
    }

    let mut my_map = TestMap {
        entries: Vec::with_capacity(1),
        indices: Vec::new(),
    };

    my_map.try_reserve(2).unwrap();
}


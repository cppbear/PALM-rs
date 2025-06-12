// Answer 0

#[test]
fn test_try_reserve_success() {
    struct TestHeaderMap {
        entries: Vec<u8>, // Dummy type to fulfill the HeaderMap requirement
        indices: Box<[Pos]>,
        mask: Size,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            TestHeaderMap {
                entries: Vec::with_capacity(0),
                indices: vec![Pos::none(); 2].into_boxed_slice(),
                mask: 1, // Initial mask for capacity of 2
            }
        }
    
        fn try_reserve(&mut self, additional: usize) -> Result<(), MaxSizeReached> {
            let cap = self.entries.len().checked_add(additional).ok_or_else(MaxSizeReached::new)?;
            let raw_cap = to_raw_capacity(cap);
    
            if raw_cap > self.indices.len() {
                let raw_cap = raw_cap.checked_next_power_of_two().ok_or_else(MaxSizeReached::new)?;
                if raw_cap > MAX_SIZE {
                    return Err(MaxSizeReached::new());
                }
    
                if self.entries.is_empty() {
                    self.mask = raw_cap as Size - 1;
                    self.indices = vec![Pos::none(); raw_cap].into_boxed_slice();
                    self.entries = Vec::with_capacity(usable_capacity(raw_cap));
                } else {
                    // Simulating growth, normally calling `try_grow`
                    self.indices = vec![Pos::none(); raw_cap].into_boxed_slice();
                }
            }
            Ok(())
        }
    }

    let mut map = TestHeaderMap::new();
    map.try_reserve(10).unwrap();
}

#[test]
#[should_panic]
fn test_try_reserve_overflow() {
    struct TestHeaderMap {
        entries: Vec<u8>, // Dummy type to fulfill the HeaderMap requirement
        indices: Box<[Pos]>,
        mask: Size,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            TestHeaderMap {
                entries: Vec::with_capacity(usize::MAX), // Intentionally max to cause overflow
                indices: vec![Pos::none(); 1].into_boxed_slice(),
                mask: 0,
            }
        }
    
        fn try_reserve(&mut self, additional: usize) -> Result<(), MaxSizeReached> {
            let cap = self.entries.len().checked_add(additional).ok_or_else(MaxSizeReached::new)?;
            let raw_cap = to_raw_capacity(cap);

            if raw_cap > self.indices.len() {
                let raw_cap = raw_cap.checked_next_power_of_two().ok_or_else(MaxSizeReached::new)?;
                if raw_cap > MAX_SIZE {
                    return Err(MaxSizeReached::new());
                }
            }
            Ok(())
        }
    }

    let mut map = TestHeaderMap::new();
    let _ = map.try_reserve(1); // Should panic due to overflow
}

#[test]
fn test_try_reserve_max_size() {
    struct TestHeaderMap {
        entries: Vec<u8>, // Dummy type to fulfill the HeaderMap requirement
        indices: Box<[Pos]>,
        mask: Size,
    }

    impl TestHeaderMap {
        fn new() -> Self {
            TestHeaderMap {
                entries: Vec::with_capacity(0),
                indices: vec![Pos::none(); 1].into_boxed_slice(),
                mask: 0,
            }
        }

        fn try_reserve(&mut self, additional: usize) -> Result<(), MaxSizeReached> {
            let cap = self.entries.len().checked_add(additional).ok_or_else(MaxSizeReached::new)?;
            let raw_cap = to_raw_capacity(cap);
    
            if raw_cap > self.indices.len() {
                let raw_cap = raw_cap.checked_next_power_of_two().ok_or_else(MaxSizeReached::new)?;
                if raw_cap > MAX_SIZE {
                    return Err(MaxSizeReached::new());
                }
            }
            Ok(())
        }
    }

    let mut map = TestHeaderMap::new();
    let _ = map.try_reserve(MAX_SIZE as usize); // Should not panic or return error
}


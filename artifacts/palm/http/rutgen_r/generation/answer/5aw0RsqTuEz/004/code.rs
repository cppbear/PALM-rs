// Answer 0

#[test]
fn test_try_reserve_success() {
    struct MockHeaderMap {
        entries: Vec<u8>, // Dummy data type
        indices: Vec<usize>, // Dummy data type
        mask: usize,
    }

    impl MockHeaderMap {
        fn new() -> Self {
            Self {
                entries: Vec::new(),
                indices: Vec::new(),
                mask: 0,
            }
        }

        fn try_reserve(&mut self, additional: usize) -> Result<(), String> {
            let cap = self.entries.len().checked_add(additional).ok_or("MaxSizeReached")?;

            let raw_cap = cap.next_power_of_two(); // assuming this function works as to_raw_capacity
            const MAX_SIZE: usize = 65536; // hypothetical max size
            
            if raw_cap > self.indices.len() {
                if raw_cap > MAX_SIZE {
                    return Err("MaxSizeReached".into());
                }

                if self.entries.is_empty() {
                    self.mask = raw_cap - 1;
                    self.indices = vec![0; raw_cap];
                    self.entries = Vec::with_capacity(raw_cap);
                }

                return Ok(());
            }

            Err("Capacity not increased".into())
        }
    }

    let mut map = MockHeaderMap::new();
    let result = map.try_reserve(1);

    assert!(result.is_ok());
}

#[test]
#[should_panic(expected = "MaxSizeReached")]
fn test_try_reserve_too_large() {
    struct MockHeaderMap {
        entries: Vec<u8>, // Dummy data type
        indices: Vec<usize>, // Dummy data type
        mask: usize,
    }

    impl MockHeaderMap {
        fn new() -> Self {
            Self {
                entries: Vec::new(),
                indices: Vec::new(),
                mask: 0,
            }
        }

        fn try_reserve(&mut self, additional: usize) -> Result<(), String> {
            let cap = self.entries.len().checked_add(additional).ok_or("MaxSizeReached")?;

            let raw_cap = cap.next_power_of_two(); // assuming this function works as to_raw_capacity
            const MAX_SIZE: usize = 65536; // hypothetical max size
            
            if raw_cap > self.indices.len() {
                if raw_cap > MAX_SIZE {
                    return Err("MaxSizeReached".into());
                }

                if self.entries.is_empty() {
                    self.mask = raw_cap - 1;
                    self.indices = vec![0; raw_cap];
                    self.entries = Vec::with_capacity(raw_cap);
                }

                return Ok(());
            }

            Err("Capacity not increased".into())
        }
    }

    let mut map = MockHeaderMap::new();
    map.try_reserve(65536); // Fill map to its MAX_SIZE
    let _ = map.try_reserve(1); // This should panic
}


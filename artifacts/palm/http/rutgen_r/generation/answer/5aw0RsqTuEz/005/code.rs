// Answer 0

#[test]
fn test_try_reserve_success() {
    struct HeaderMap {
        entries: Vec<u8>,
        indices: Vec<usize>,
        mask: usize,
    }

    struct MaxSizeReached;

    impl MaxSizeReached {
        fn new() -> Self {
            MaxSizeReached
        }
    }

    const MAX_SIZE: usize = 1024;

    impl HeaderMap {
        fn new() -> Self {
            HeaderMap {
                entries: Vec::new(),
                indices: Vec::new(),
                mask: 0,
            }
        }

        fn try_reserve(&mut self, additional: usize) -> Result<(), MaxSizeReached> {
            let cap = self.entries.len().checked_add(additional).ok_or_else(MaxSizeReached::new)?;
            let raw_cap = cap.next_power_of_two();

            if raw_cap > self.indices.len() {
                let raw_cap = raw_cap.checked_next_power_of_two().ok_or_else(MaxSizeReached::new)?;
                if raw_cap > MAX_SIZE {
                    return Err(MaxSizeReached::new());
                }

                if self.entries.is_empty() {
                    self.mask = raw_cap - 1;
                    self.indices = vec![0; raw_cap];
                    self.entries = Vec::with_capacity(raw_cap);
                } else {
                    return Err(MaxSizeReached::new());
                }
            }

            Ok(())
        }

        fn fill_entries(&mut self, count: usize) {
            self.entries = (0..count).map(|i| i as u8).collect();
            self.indices = vec![0; self.entries.len().next_power_of_two()];
        }
    }

    let mut map = HeaderMap::new();
    map.fill_entries(5); // Entries count is 5, not empty

    // Try reserving additional space
    let result = map.try_reserve(10);
    assert_eq!(result, Ok(()));
}


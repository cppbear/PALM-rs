// Answer 0

#[test]
fn test_try_reserve_success_case() {
    struct HeaderMap {
        entries: Vec<u8>,
        indices: Box<[u8]>,
        mask: usize,
    }

    impl HeaderMap {
        fn new() -> Self {
            HeaderMap {
                entries: Vec::new(),
                indices: Box::new([]),
                mask: 0,
            }
        }

        fn try_reserve(&mut self, additional: usize) -> Result<(), MaxSizeReached> {
            let cap = self.entries.len().checked_add(additional).ok_or_else(MaxSizeReached::new)?;

            let raw_cap = to_raw_capacity(cap);

            if raw_cap > self.indices.len() {
                let raw_cap = raw_cap
                    .checked_next_power_of_two()
                    .ok_or_else(MaxSizeReached::new)?;
                if raw_cap > MAX_SIZE {
                    return Err(MaxSizeReached::new());
                }

                if self.entries.is_empty() {
                    self.mask = raw_cap as Size - 1;
                    self.indices = vec![0; raw_cap].into_boxed_slice();
                    self.entries = Vec::with_capacity(usable_capacity(raw_cap));
                } else {
                    self.try_grow(raw_cap)?;
                }
            }

            Ok(())
        }
    }

    struct MaxSizeReached;

    impl MaxSizeReached {
        fn new() -> Self {
            MaxSizeReached
        }
    }

    fn to_raw_capacity(cap: usize) -> usize {
        cap
    }

    const MAX_SIZE: usize = 1024;
    type Size = usize;

    let mut map = HeaderMap::new();
    map.indices = vec![0; 8].into_boxed_slice(); // initial length of indices
    assert!(map.try_reserve(8).is_ok()); // Test with additional that does not exceed capacity
}

#[test]
fn test_try_reserve_exceed_capacity() {
    struct HeaderMap {
        entries: Vec<u8>,
        indices: Box<[u8]>,
        mask: usize,
    }

    impl HeaderMap {
        fn new() -> Self {
            HeaderMap {
                entries: Vec::new(),
                indices: Box::new([]),
                mask: 0,
            }
        }

        fn try_reserve(&mut self, additional: usize) -> Result<(), MaxSizeReached> {
            let cap = self.entries.len().checked_add(additional).ok_or_else(MaxSizeReached::new)?;

            let raw_cap = to_raw_capacity(cap);

            if raw_cap > self.indices.len() {
                let raw_cap = raw_cap
                    .checked_next_power_of_two()
                    .ok_or_else(MaxSizeReached::new)?;
                if raw_cap > MAX_SIZE {
                    return Err(MaxSizeReached::new());
                }

                if self.entries.is_empty() {
                    self.mask = raw_cap as Size - 1;
                    self.indices = vec![0; raw_cap].into_boxed_slice();
                    self.entries = Vec::with_capacity(usable_capacity(raw_cap));
                } else {
                    self.try_grow(raw_cap)?;
                }
            }

            Ok(())
        }
    }

    struct MaxSizeReached;

    impl MaxSizeReached {
        fn new() -> Self {
            MaxSizeReached
        }
    }

    fn to_raw_capacity(cap: usize) -> usize {
        cap
    }

    const MAX_SIZE: usize = 1024;
    type Size = usize;

    let mut map = HeaderMap::new();
    map.indices = vec![0; 8].into_boxed_slice(); // initial length of indices
    assert!(map.try_reserve(16).is_err()); // This should exceed capacity and return an error
}


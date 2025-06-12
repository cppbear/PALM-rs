// Answer 0

fn test_try_grow_success() {
    struct TestHeaderMap {
        mask: Size,
        indices: Box<[Pos]>,
        entries: Vec<Bucket<HeaderValue>>,
        extra_values: Vec<ExtraValue<HeaderValue>>,
    }

    impl TestHeaderMap {
        fn new(capacity: usize) -> Self {
            let indices = vec![Pos::new(0, HashValue(0)); capacity].into_boxed_slice();
            TestHeaderMap {
                mask: (capacity - 1) as Size,
                indices,
                entries: Vec::new(),
                extra_values: Vec::new(),
            }
        }

        fn reinsert_entry_in_order(&mut self, _pos: Pos) {
            // Simplified for the test context, does nothing.
        }

        fn try_grow(&mut self, new_raw_cap: usize) -> Result<(), MaxSizeReached> {
            if new_raw_cap > MAX_SIZE {
                return Err(MaxSizeReached::new());
            }

            let mut first_ideal = 0;

            for (i, pos) in self.indices.iter().enumerate() {
                if let Some((_, _entry_hash)) = pos.resolve() {
                    if probe_distance(self.mask, HashValue(0), i) == 0 {
                        first_ideal = i;
                        break;
                    }
                }
            }

            let old_indices = mem::replace(
                &mut self.indices,
                vec![Pos::none(); new_raw_cap].into_boxed_slice(),
            );
            self.mask = new_raw_cap.wrapping_sub(1) as Size;

            for &pos in &old_indices[first_ideal..] {
                self.reinsert_entry_in_order(pos);
            }

            for &pos in &old_indices[..first_ideal] {
                self.reinsert_entry_in_order(pos);
            }

            Ok(())
        }
    }

    let mut header_map = TestHeaderMap::new(16);
    let result = header_map.try_grow(16);

    assert_eq!(result, Ok(()));
}

fn test_try_grow_max_size_error() {
    struct TestHeaderMap {
        mask: Size,
        indices: Box<[Pos]>,
        entries: Vec<Bucket<HeaderValue>>,
        extra_values: Vec<ExtraValue<HeaderValue>>,
    }

    impl TestHeaderMap {
        fn new(capacity: usize) -> Self {
            let indices = vec![Pos::new(0, HashValue(0)); capacity].into_boxed_slice();
            TestHeaderMap {
                mask: (capacity - 1) as Size,
                indices,
                entries: Vec::new(),
                extra_values: Vec::new(),
            }
        }

        fn try_grow(&mut self, new_raw_cap: usize) -> Result<(), MaxSizeReached> {
            if new_raw_cap > MAX_SIZE {
                return Err(MaxSizeReached::new());
            }
            Ok(())
        }
    }

    let mut header_map = TestHeaderMap::new(16);
    let result = header_map.try_grow(32768); // MAX_SIZE is smaller than this

    assert_eq!(result.is_err(), true);
}

fn test_try_grow_ideal_position() {
    struct TestHeaderMap {
        mask: Size,
        indices: Box<[Pos]>,
        entries: Vec<Bucket<HeaderValue>>,
        extra_values: Vec<ExtraValue<HeaderValue>>,
    }

    impl TestHeaderMap {
        fn new(capacity: usize) -> Self {
            let indices = vec![Pos::new(0, HashValue(0)); capacity].into_boxed_slice();
            TestHeaderMap {
                mask: (capacity - 1) as Size,
                indices,
                entries: Vec::new(),
                extra_values: Vec::new(),
            }
        }

        fn reinsert_entry_in_order(&mut self, _pos: Pos) {
            // Simplified for the test context, does nothing.
        }

        fn try_grow(&mut self, new_raw_cap: usize) -> Result<(), MaxSizeReached> {
            if new_raw_cap > MAX_SIZE {
                return Err(MaxSizeReached::new());
            }

            let mut first_ideal = 0;

            for (i, pos) in self.indices.iter().enumerate() {
                if let Some((_, _entry_hash)) = pos.resolve() {
                    if probe_distance(self.mask, HashValue(0), i) == 0 {
                        first_ideal = i;
                        break;
                    }
                }
            }

            let old_indices = mem::replace(
                &mut self.indices,
                vec![Pos::none(); new_raw_cap].into_boxed_slice(),
            );
            self.mask = new_raw_cap.wrapping_sub(1) as Size;

            for &pos in &old_indices[first_ideal..] {
                self.reinsert_entry_in_order(pos);
            }

            for &pos in &old_indices[..first_ideal] {
                self.reinsert_entry_in_order(pos);
            }

            Ok(())
        }
    }

    let mut header_map = TestHeaderMap::new(16);
    header_map.indices[0] = Pos::new(0, HashValue(0)); // Ensure ideal position condition
    let result = header_map.try_grow(32);

    assert_eq!(result, Ok(()));
}


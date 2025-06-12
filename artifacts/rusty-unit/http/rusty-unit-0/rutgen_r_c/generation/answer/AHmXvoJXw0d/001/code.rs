// Answer 0

#[test]
fn test_try_grow_exceeding_max_size() {
    struct TestHeaderMap {
        mask: Size,
        indices: Box<[Pos]>,
        entries: Vec<Bucket<HeaderValue>>,
        danger: Danger,
    }

    impl TestHeaderMap {
        fn new(capacity: usize) -> Self {
            TestHeaderMap {
                mask: capacity as Size,
                indices: vec![Pos::none(); capacity].into_boxed_slice(),
                entries: Vec::new(),
                danger: Danger::Green,
            }
        }

        fn try_grow(&mut self, new_raw_cap: usize) -> Result<(), MaxSizeReached> {
            if new_raw_cap > MAX_SIZE {
                return Err(MaxSizeReached::new());
            }

            // find first ideally placed element -- start of cluster
            let mut first_ideal = 0;

            for (i, pos) in self.indices.iter().enumerate() {
                if let Some((_, entry_hash)) = pos.resolve() {
                    if 0 == probe_distance(self.mask, entry_hash, i) {
                        first_ideal = i;
                        break;
                    }
                }
            }

            // visit the entries in an order where we can simply reinsert them
            // into self.indices without any bucket stealing.
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

            // Reserve additional entry slots
            let more = self.capacity() - self.entries.len();
            self.entries.reserve_exact(more);
            Ok(())
        }

        fn reinsert_entry_in_order(&mut self, _pos: Pos) {}
        fn capacity(&self) -> usize {
            self.indices.len()
        }
    }

    let mut header_map = TestHeaderMap::new(16);
    let new_capacity = 32768; // greater than MAX_SIZE (which is 32768)
    
    assert!(header_map.try_grow(new_capacity).is_err());
}


// Answer 0

#[test]
fn test_try_grow_success() {
    struct TestHeaderMap {
        mask: Size,
        indices: Box<[Pos]>,
        entries: Vec<Bucket<i32>>,
        danger: Danger,
    }

    impl TestHeaderMap {
        fn new(initial_capacity: usize) -> Self {
            let mask = initial_capacity.wrapping_sub(1) as Size;
            let indices = vec![Pos::none(); initial_capacity].into_boxed_slice();
            let entries = Vec::with_capacity(initial_capacity);
            let danger = Danger::Green;
            TestHeaderMap { mask, indices, entries, danger }
        }

        fn capacity(&self) -> usize {
            self.indices.len()
        }

        fn reserve(&mut self, additional: usize) {
            self.entries.reserve(additional);
        }
    }

    let mut map = TestHeaderMap::new(8);
    let result = map.try_grow(16);
    assert!(result.is_ok());
}

#[test]
fn test_try_grow_exceeds_max_size() {
    struct TestHeaderMap {
        mask: Size,
        indices: Box<[Pos]>,
        entries: Vec<Bucket<i32>>,
        danger: Danger,
    }

    impl TestHeaderMap {
        fn new(initial_capacity: usize) -> Self {
            let mask = initial_capacity.wrapping_sub(1) as Size;
            let indices = vec![Pos::none(); initial_capacity].into_boxed_slice();
            let entries = Vec::with_capacity(initial_capacity);
            let danger = Danger::Green;
            TestHeaderMap { mask, indices, entries, danger }
        }

        fn capacity(&self) -> usize {
            self.indices.len()
        }

        fn reserve(&mut self, additional: usize) {
            self.entries.reserve(additional);
        }
    }

    let mut map = TestHeaderMap::new(8);
    let result = map.try_grow(1 << 16); // Exceeds MAX_SIZE
    assert!(result.is_err());
}


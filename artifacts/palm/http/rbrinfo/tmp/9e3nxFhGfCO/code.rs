fn new(index: usize, hash: HashValue) -> Self {
        debug_assert!(index < MAX_SIZE);
        Pos {
            index: index as Size,
            hash,
        }
    }
pub fn next_index(&mut self) -> usize {
        let next_n = self.n + 1;

        // There's room for further optimisation here:
        // random_range uses rejection sampling (or other method; see #1196) to avoid bias.
        // When the initial sample is biased for range 0..bound
        // it may still be viable to use for a smaller bound
        // (especially if small biases are considered acceptable).

        let next_chunk_remaining = self.chunk_remaining.checked_sub(1).unwrap_or_else(|| {
            // If the chunk is empty, generate a new chunk
            let (bound, remaining) = calculate_bound_u32(next_n);
            // bound = (n + 1) * (n + 2) *..* (n + remaining)
            self.chunk = self.rng.random_range(..bound);
            // Chunk is a random number in
            // [0, (n + 1) * (n + 2) *..* (n + remaining) )

            remaining - 1
        });

        let result = if next_chunk_remaining == 0 {
            // `chunk` is a random number in the range [0..n+1)
            // Because `chunk_remaining` is about to be set to zero
            // we do not need to clear the chunk here
            self.chunk as usize
        } else {
            // `chunk` is a random number in a range that is a multiple of n+1
            // so r will be a random number in [0..n+1)
            let r = self.chunk % next_n;
            self.chunk /= next_n;
            r as usize
        };

        self.chunk_remaining = next_chunk_remaining;
        self.n = next_n;
        result
    }
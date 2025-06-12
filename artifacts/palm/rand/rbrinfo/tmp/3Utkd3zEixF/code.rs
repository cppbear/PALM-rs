pub fn new(rng: R, n: u32) -> Self {
        // If n = 0, the first number returned will always be 0
        // so we don't need to generate a random number
        let chunk_remaining = if n == 0 { 1 } else { 0 };
        Self {
            rng,
            n,
            chunk: 0,
            chunk_remaining,
        }
    }
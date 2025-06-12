fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        const RANGE: u8 = 26 + 26;

        let offset = rng.random_range(0..RANGE) + b'A';

        // Account for upper-cases
        offset + (offset > b'Z') as u8 * (b'a' - b'Z' - 1)
    }
fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u128 {
        // Use LE; we explicitly generate one value before the next.
        let x = u128::from(rng.next_u64());
        let y = u128::from(rng.next_u64());
        (y << 64) | x
    }
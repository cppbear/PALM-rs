fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u8 {
        rng.next_u32() as u8
    }
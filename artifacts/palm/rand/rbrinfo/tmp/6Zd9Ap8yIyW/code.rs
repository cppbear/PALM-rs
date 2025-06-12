fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> u16 {
        rng.next_u32() as u16
    }
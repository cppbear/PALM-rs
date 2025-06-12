fn choose_multiple_array<R, const N: usize>(&self, rng: &mut R) -> Option<[Self::Output; N]>
    where
        Self::Output: Clone + Sized,
        R: Rng + ?Sized,
    {
        let indices = index::sample_array(rng, self.len())?;
        Some(indices.map(|index| self[index].clone()))
    }
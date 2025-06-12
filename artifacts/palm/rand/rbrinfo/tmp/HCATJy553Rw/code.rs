fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> usize {
        let chosen_weight = self.weight_distribution.sample(rng);
        // Find the first item which has a weight *higher* than the chosen weight.
        self.cumulative_weights
            .partition_point(|w| w <= &chosen_weight)
    }
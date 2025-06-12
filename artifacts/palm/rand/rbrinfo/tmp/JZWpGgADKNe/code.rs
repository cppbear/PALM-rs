fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Duration {
        match self.mode {
            UniformDurationMode::Small { secs, nanos } => {
                let n = nanos.sample(rng);
                Duration::new(secs, n)
            }
            UniformDurationMode::Medium { nanos } => {
                let nanos = nanos.sample(rng);
                Duration::new(nanos / 1_000_000_000, (nanos % 1_000_000_000) as u32)
            }
            UniformDurationMode::Large {
                max_secs,
                max_nanos,
                secs,
            } => {
                // constant folding means this is at least as fast as `Rng::sample(Range)`
                let nano_range = Uniform::new(0, 1_000_000_000).unwrap();
                loop {
                    let s = secs.sample(rng);
                    let n = nano_range.sample(rng);
                    if !(s == max_secs && n > max_nanos) {
                        let sum = n + self.offset;
                        break Duration::new(s, sum);
                    }
                }
            }
        }
    }
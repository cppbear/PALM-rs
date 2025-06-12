fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Result<Self, Error>
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        let low = *low_b.borrow();
        let high = *high_b.borrow();
        if !(low <= high) {
            return Err(Error::EmptyRange);
        }

        let low_s = low.as_secs();
        let low_n = low.subsec_nanos();
        let mut high_s = high.as_secs();
        let mut high_n = high.subsec_nanos();

        if high_n < low_n {
            high_s -= 1;
            high_n += 1_000_000_000;
        }

        let mode = if low_s == high_s {
            UniformDurationMode::Small {
                secs: low_s,
                nanos: Uniform::new_inclusive(low_n, high_n)?,
            }
        } else {
            let max = high_s
                .checked_mul(1_000_000_000)
                .and_then(|n| n.checked_add(u64::from(high_n)));

            if let Some(higher_bound) = max {
                let lower_bound = low_s * 1_000_000_000 + u64::from(low_n);
                UniformDurationMode::Medium {
                    nanos: Uniform::new_inclusive(lower_bound, higher_bound)?,
                }
            } else {
                // An offset is applied to simplify generation of nanoseconds
                let max_nanos = high_n - low_n;
                UniformDurationMode::Large {
                    max_secs: high_s,
                    max_nanos,
                    secs: Uniform::new_inclusive(low_s, high_s)?,
                }
            }
        };
        Ok(UniformDuration {
            mode,
            offset: low_n,
        })
    }
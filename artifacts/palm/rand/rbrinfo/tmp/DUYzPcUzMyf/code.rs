pub fn new<I>(weights: I) -> Result<WeightedIndex<X>, Error>
    where
        I: IntoIterator,
        I::Item: SampleBorrow<X>,
        X: Weight,
    {
        let mut iter = weights.into_iter();
        let mut total_weight: X = iter.next().ok_or(Error::InvalidInput)?.borrow().clone();

        let zero = X::ZERO;
        if !(total_weight >= zero) {
            return Err(Error::InvalidWeight);
        }

        let mut weights = Vec::<X>::with_capacity(iter.size_hint().0);
        for w in iter {
            // Note that `!(w >= x)` is not equivalent to `w < x` for partially
            // ordered types due to NaNs which are equal to nothing.
            if !(w.borrow() >= &zero) {
                return Err(Error::InvalidWeight);
            }
            weights.push(total_weight.clone());

            if let Err(()) = total_weight.checked_add_assign(w.borrow()) {
                return Err(Error::Overflow);
            }
        }

        if total_weight == zero {
            return Err(Error::InsufficientNonZero);
        }
        let distr = X::Sampler::new(zero, total_weight.clone()).unwrap();

        Ok(WeightedIndex {
            cumulative_weights: weights,
            total_weight,
            weight_distribution: distr,
        })
    }
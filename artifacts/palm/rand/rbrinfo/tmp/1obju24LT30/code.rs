fn choose<R>(mut self, rng: &mut R) -> Option<Self::Item>
    where
        R: Rng + ?Sized,
    {
        let (mut lower, mut upper) = self.size_hint();
        let mut result = None;

        // Handling for this condition outside the loop allows the optimizer to eliminate the loop
        // when the Iterator is an ExactSizeIterator. This has a large performance impact on e.g.
        // seq_iter_choose_from_1000.
        if upper == Some(lower) {
            return match lower {
                0 => None,
                1 => self.next(),
                _ => self.nth(rng.random_range(..lower)),
            };
        }

        let mut coin_flipper = CoinFlipper::new(rng);
        let mut consumed = 0;

        // Continue until the iterator is exhausted
        loop {
            if lower > 1 {
                let ix = coin_flipper.rng.random_range(..lower + consumed);
                let skip = if ix < lower {
                    result = self.nth(ix);
                    lower - (ix + 1)
                } else {
                    lower
                };
                if upper == Some(lower) {
                    return result;
                }
                consumed += lower;
                if skip > 0 {
                    self.nth(skip - 1);
                }
            } else {
                let elem = self.next();
                if elem.is_none() {
                    return result;
                }
                consumed += 1;
                if coin_flipper.random_ratio_one_over(consumed) {
                    result = elem;
                }
            }

            let hint = self.size_hint();
            lower = hint.0;
            upper = hint.1;
        }
    }
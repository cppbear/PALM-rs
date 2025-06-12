fn choose_stable<R>(mut self, rng: &mut R) -> Option<Self::Item>
    where
        R: Rng + ?Sized,
    {
        let mut consumed = 0;
        let mut result = None;
        let mut coin_flipper = CoinFlipper::new(rng);

        loop {
            // Currently the only way to skip elements is `nth()`. So we need to
            // store what index to access next here.
            // This should be replaced by `advance_by()` once it is stable:
            // https://github.com/rust-lang/rust/issues/77404
            let mut next = 0;

            let (lower, _) = self.size_hint();
            if lower >= 2 {
                let highest_selected = (0..lower)
                    .filter(|ix| coin_flipper.random_ratio_one_over(consumed + ix + 1))
                    .last();

                consumed += lower;
                next = lower;

                if let Some(ix) = highest_selected {
                    result = self.nth(ix);
                    next -= ix + 1;
                    debug_assert!(result.is_some(), "iterator shorter than size_hint().0");
                }
            }

            let elem = self.nth(next);
            if elem.is_none() {
                return result;
            }

            if coin_flipper.random_ratio_one_over(consumed + 1) {
                result = elem;
            }
            consumed += 1;
        }
    }
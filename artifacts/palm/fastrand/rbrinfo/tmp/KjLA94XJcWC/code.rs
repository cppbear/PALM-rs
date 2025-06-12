pub fn choose_multiple<I: IntoIterator>(&mut self, source: I, amount: usize) -> Vec<I::Item> {
        // Adapted from: https://docs.rs/rand/latest/rand/seq/trait.IteratorRandom.html#method.choose_multiple
        let mut reservoir = Vec::with_capacity(amount);
        let mut iter = source.into_iter();

        reservoir.extend(iter.by_ref().take(amount));

        // Continue unless the iterator was exhausted
        //
        // note: this prevents iterators that "restart" from causing problems.
        // If the iterator stops once, then so do we.
        if reservoir.len() == amount {
            for (i, elem) in iter.enumerate() {
                let end = i + 1 + amount;
                let k = self.usize(0..end);
                if let Some(slot) = reservoir.get_mut(k) {
                    *slot = elem;
                }
            }
        } else {
            // If less than one third of the `Vec` was used, reallocate
            // so that the unused space is not wasted. There is a corner
            // case where `amount` was much less than `self.len()`.
            if reservoir.capacity() > 3 * reservoir.len() {
                reservoir.shrink_to_fit();
            }
        }
        reservoir
    }
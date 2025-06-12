fn choose_multiple_fill<R>(mut self, rng: &mut R, buf: &mut [Self::Item]) -> usize
    where
        R: Rng + ?Sized,
    {
        let amount = buf.len();
        let mut len = 0;
        while len < amount {
            if let Some(elem) = self.next() {
                buf[len] = elem;
                len += 1;
            } else {
                // Iterator exhausted; stop early
                return len;
            }
        }

        // Continue, since the iterator was not exhausted
        for (i, elem) in self.enumerate() {
            let k = rng.random_range(..i + 1 + amount);
            if let Some(slot) = buf.get_mut(k) {
                *slot = elem;
            }
        }
        len
    }
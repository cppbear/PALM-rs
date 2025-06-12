fn retain2<F>(&mut self, keep: F)
    where
        F: FnMut(&mut K, &mut V) -> bool,
    {
        self.core.retain_in_order(keep);
    }
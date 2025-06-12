pub fn try_reclaim(&mut self, additional: usize) -> bool {
        let len = self.len();
        let rem = self.capacity() - len;

        if additional <= rem {
            // The handle can already store at least `additional` more bytes, so
            // there is no further work needed to be done.
            return true;
        }

        self.reserve_inner(additional, false)
    }
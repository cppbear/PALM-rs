fn next(&mut self) -> Option<Option<(usize, usize)>> {
        if self.idx >= self.locs.len() {
            return None;
        }
        let x = match self.locs.pos(self.idx) {
            None => Some(None),
            Some((s, e)) => {
                Some(Some((s, e)))
            }
        };
        self.idx += 1;
        x
    }
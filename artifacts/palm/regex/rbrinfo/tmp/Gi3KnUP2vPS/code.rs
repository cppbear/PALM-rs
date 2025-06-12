fn next(&mut self) -> Option<&'t [u8]> {
        if self.n == 0 {
            return None
        }
        self.n -= 1;
        if self.n == 0 {
            let text = self.splits.finder.0.text();
            Some(&text[self.splits.last..])
        } else {
            self.splits.next()
        }
    }
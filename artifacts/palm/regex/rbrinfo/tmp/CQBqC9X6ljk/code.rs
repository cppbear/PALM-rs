fn next(&mut self) -> Option<&'t str> {
        let text = self.finder.0.text();
        match self.finder.next() {
            None => {
                if self.last >= text.len() {
                    None
                } else {
                    let s = &text[self.last..];
                    self.last = text.len();
                    Some(s)
                }
            }
            Some(m) => {
                let matched = &text[self.last..m.start()];
                self.last = m.end();
                Some(matched)
            }
        }
    }
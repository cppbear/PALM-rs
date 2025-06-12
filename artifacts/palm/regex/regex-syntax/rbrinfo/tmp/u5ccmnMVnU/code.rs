fn next(&mut self) -> Option<char> {
        self.0.next().map(|c| *c)
    }
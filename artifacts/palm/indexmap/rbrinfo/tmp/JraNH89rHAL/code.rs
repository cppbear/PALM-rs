fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|x| (x, ()))
    }
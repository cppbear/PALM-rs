pub fn reverse(mut self, yes: bool) -> Self {
        self.compiled.is_reverse = yes;
        self
    }
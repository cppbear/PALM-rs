pub fn only_utf8(mut self, yes: bool) -> Self {
        self.compiled.only_utf8 = yes;
        self
    }
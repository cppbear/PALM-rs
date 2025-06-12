pub fn dfa(mut self, yes: bool) -> Self {
        self.compiled.is_dfa = yes;
        self
    }
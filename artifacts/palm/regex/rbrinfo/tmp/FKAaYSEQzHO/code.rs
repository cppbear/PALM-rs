pub fn capture_names(&self) -> &[Option<String>] {
        &self.ro.nfa.captures
    }
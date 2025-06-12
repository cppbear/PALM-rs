fn prefix_at(&self, text: &[u8], at: usize) -> Option<usize> {
        self.prog.prefixes.find(&text[at..]).map(|(s, _)| at + s)
    }
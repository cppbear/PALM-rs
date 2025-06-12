pub fn find_end(&self, haystack: &[u8]) -> Option<(usize, usize)> {
        for lit in self.iter() {
            if lit.len() > haystack.len() {
                continue;
            }
            if lit == &haystack[haystack.len() - lit.len()..] {
                return Some((haystack.len() - lit.len(), haystack.len()));
            }
        }
        None
    }
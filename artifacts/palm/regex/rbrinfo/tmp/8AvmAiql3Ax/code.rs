fn index(&self, i: usize) -> &[u8] {
        self.get(i).map(|m| m.as_bytes())
            .unwrap_or_else(|| panic!("no group at index '{}'", i))
    }
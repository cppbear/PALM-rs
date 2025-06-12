fn index(&self, i: usize) -> &str {
        self.get(i).map(|m| m.as_str())
            .unwrap_or_else(|| panic!("no group at index '{}'", i))
    }
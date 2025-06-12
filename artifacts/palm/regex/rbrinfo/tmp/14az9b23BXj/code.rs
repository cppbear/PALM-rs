fn index<'a>(&'a self, name: &'i str) -> &'a str {
        self.name(name).map(|m| m.as_str())
            .unwrap_or_else(|| panic!("no group named '{}'", name))
    }
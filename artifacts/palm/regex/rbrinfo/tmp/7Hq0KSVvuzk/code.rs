fn index<'a>(&'a self, name: &'i str) -> &'a [u8] {
        self.name(name).map(|m| m.as_bytes())
            .unwrap_or_else(|| panic!("no group named '{}'", name))
    }
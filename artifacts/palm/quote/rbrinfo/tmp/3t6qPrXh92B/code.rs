fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let id = self.to_string();
        if let Some(id) = id.strip_prefix("r#") {
            fmt::Display::fmt(id, f)
        } else {
            fmt::Display::fmt(&id[..], f)
        }
    }
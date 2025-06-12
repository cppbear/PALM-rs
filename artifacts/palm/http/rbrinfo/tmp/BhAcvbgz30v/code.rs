fn eq(&self, other: &Authority) -> bool {
        self.eq_ignore_ascii_case(other.as_str())
    }
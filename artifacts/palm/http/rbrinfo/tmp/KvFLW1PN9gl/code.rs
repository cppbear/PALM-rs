fn eq(&self, other: &Authority) -> bool {
        self.as_str().eq_ignore_ascii_case(other.as_str())
    }
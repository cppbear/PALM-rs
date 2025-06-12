fn eq(&self, other: &str) -> bool {
        eq_ignore_ascii_case(self.as_ref(), other.as_bytes())
    }
fn eq(&self, other: &str) -> bool {
        self.as_str().eq_ignore_ascii_case(other)
    }
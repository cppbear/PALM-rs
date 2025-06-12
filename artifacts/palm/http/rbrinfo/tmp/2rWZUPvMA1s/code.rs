fn eq(&self, other: &str) -> bool {
        self.data.eq_ignore_ascii_case(other)
    }
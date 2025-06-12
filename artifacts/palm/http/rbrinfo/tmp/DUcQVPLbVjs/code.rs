fn eq(&self, other: &String) -> bool {
        self.data.eq_ignore_ascii_case(other.as_str())
    }
fn eq(&self, other: &Authority) -> bool {
        self.data.eq_ignore_ascii_case(&other.data)
    }
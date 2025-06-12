fn eq(&self, other: &&'a str) -> bool {
        self.data.eq_ignore_ascii_case(other)
    }
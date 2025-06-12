fn partial_cmp(&self, other: &&'a str) -> Option<cmp::Ordering> {
        self.as_str().partial_cmp(*other)
    }
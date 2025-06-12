fn partial_cmp(&self, other: &Authority) -> Option<cmp::Ordering> {
        let left = self.data.as_bytes().iter().map(|b| b.to_ascii_lowercase());
        let right = other.data.as_bytes().iter().map(|b| b.to_ascii_lowercase());
        left.partial_cmp(right)
    }
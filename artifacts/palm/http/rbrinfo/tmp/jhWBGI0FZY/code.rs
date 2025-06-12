fn partial_cmp(&self, other: &HeaderValue) -> Option<cmp::Ordering> {
        (**self).partial_cmp(other)
    }
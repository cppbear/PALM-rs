fn eq(&self, other: &str) -> bool {
        &**self == other.as_bytes()
    }
fn eq(&self, other: &StatusCode) -> bool {
        *self == other.as_u16()
    }
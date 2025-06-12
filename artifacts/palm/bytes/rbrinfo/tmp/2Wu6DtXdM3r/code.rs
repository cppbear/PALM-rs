fn eq(&self, other: &String) -> bool {
        *self == other[..]
    }
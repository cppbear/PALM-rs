fn eq(&self, other: &Vec<u8>) -> bool {
        *self == other[..]
    }
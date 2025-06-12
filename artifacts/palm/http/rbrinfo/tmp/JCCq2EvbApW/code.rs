fn eq(&self, other: &&'a T) -> bool {
        *self == **other
    }
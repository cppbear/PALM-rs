fn eq(&self, other: &&'a HeaderName) -> bool {
        *self == **other
    }
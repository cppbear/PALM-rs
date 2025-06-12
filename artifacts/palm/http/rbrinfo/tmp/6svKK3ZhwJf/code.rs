fn eq(&self, other: &&'a str) -> bool {
        *self == **other
    }
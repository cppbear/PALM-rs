fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
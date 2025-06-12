fn try_from(t: &'a str) -> Result<Self, Self::Error> {
        t.parse()
    }
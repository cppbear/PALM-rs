fn try_from(t: &'a String) -> Result<Self, Self::Error> {
        t.parse()
    }
fn into_iter(self) -> Self::IntoIter {
        SetMatchesIntoIter(self.matches.into_iter().enumerate())
    }
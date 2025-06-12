pub fn iter(&self) -> SetMatchesIter {
        SetMatchesIter((&*self.matches).into_iter().enumerate())
    }
fn quote_into_iter(&'q self) -> (Self::Iter, HasIter) {
            (self.iter(), HasIter)
        }
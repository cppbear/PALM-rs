fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
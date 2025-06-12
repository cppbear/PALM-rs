fn into_iter(self) -> Self::IntoIter {
        Iter {
            iter: self.map.iter(),
        }
    }
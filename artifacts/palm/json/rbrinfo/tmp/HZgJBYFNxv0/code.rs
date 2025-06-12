fn into_iter(self) -> Self::IntoIter {
        IterMut {
            iter: self.map.iter_mut(),
        }
    }
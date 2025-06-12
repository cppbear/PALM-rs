pub fn iter(&self) -> Iter {
        Iter {
            iter: self.map.iter(),
        }
    }
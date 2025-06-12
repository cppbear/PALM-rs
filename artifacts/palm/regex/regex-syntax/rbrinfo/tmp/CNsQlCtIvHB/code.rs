pub fn iter(&self) -> ClassBytesIter {
        ClassBytesIter(self.set.iter())
    }
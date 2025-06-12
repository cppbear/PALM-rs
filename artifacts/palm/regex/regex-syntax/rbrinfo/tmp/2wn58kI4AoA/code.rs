pub fn iter(&self) -> ClassUnicodeIter {
        ClassUnicodeIter(self.set.iter())
    }
pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            map: self,
            entry: 0,
            cursor: self.entries.first().map(|_| Cursor::Head),
        }
    }
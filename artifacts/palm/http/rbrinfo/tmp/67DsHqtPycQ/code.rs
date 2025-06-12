pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            map: self as *mut _,
            entry: 0,
            cursor: self.entries.first().map(|_| Cursor::Head),
            lt: PhantomData,
        }
    }
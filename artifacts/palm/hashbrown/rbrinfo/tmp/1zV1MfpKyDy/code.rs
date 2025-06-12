pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            inner: unsafe { self.raw.iter() },
            marker: PhantomData,
        }
    }
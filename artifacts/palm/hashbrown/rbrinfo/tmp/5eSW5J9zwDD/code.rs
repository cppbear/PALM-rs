pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            inner: unsafe { self.raw.iter() },
            marker: PhantomData,
        }
    }
pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        // Here we tie the lifetime of self to the iter.
        unsafe {
            IterMut {
                inner: self.table.iter(),
                marker: PhantomData,
            }
        }
    }
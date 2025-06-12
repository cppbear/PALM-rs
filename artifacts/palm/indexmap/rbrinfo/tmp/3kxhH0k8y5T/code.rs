fn borrow_mut(&mut self) -> RefMut<'_, K, V> {
        RefMut::new(&mut self.indices, &mut self.entries)
    }
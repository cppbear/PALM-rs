pub(crate) fn next<F>(&mut self, mut f: F) -> Option<T>
    where
        F: FnMut(&mut T) -> bool,
    {
        unsafe {
            for item in &mut self.iter {
                if f(item.as_mut()) {
                    return Some(self.table.remove(item).0);
                }
            }
        }
        None
    }
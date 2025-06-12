pub unsafe fn iter(&self) -> RawIter<T> {
        // SAFETY:
        // 1. The caller must uphold the safety contract for `iter` method.
        // 2. The [`RawTableInner`] must already have properly initialized control bytes since
        //    we will never expose RawTable::new_uninitialized in a public API.
        self.table.iter()
    }
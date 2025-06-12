pub unsafe fn drain_iter_from(&mut self, iter: RawIter<T>) -> RawDrain<'_, T, A> {
        debug_assert_eq!(iter.len(), self.len());
        RawDrain {
            iter,
            table: mem::replace(&mut self.table, RawTableInner::NEW),
            orig_table: NonNull::from(&mut self.table),
            marker: PhantomData,
        }
    }
pub unsafe fn into_iter_from(self, iter: RawIter<T>) -> RawIntoIter<T, A> {
        debug_assert_eq!(iter.len(), self.len());

        let allocation = self.into_allocation();
        RawIntoIter {
            iter,
            allocation,
            marker: PhantomData,
        }
    }
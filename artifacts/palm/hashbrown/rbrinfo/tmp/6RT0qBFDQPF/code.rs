unsafe fn new<A: Allocator>(table: &RawTable<T, A>, hash: u64) -> Self {
        RawIterHash {
            inner: RawIterHashInner::new(&table.table, hash),
            _marker: PhantomData,
        }
    }
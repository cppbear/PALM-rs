pub const fn new() -> Self {
        Self {
            table: RawTableInner::NEW,
            alloc: Global,
            marker: PhantomData,
        }
    }
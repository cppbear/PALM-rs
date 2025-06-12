pub(crate) fn map_addr<T>(ptr: *mut T, f: impl FnOnce(usize) -> usize) -> *mut T
    where
        T: Sized,
    {
        self::with_addr(ptr, f(addr(ptr)))
    }
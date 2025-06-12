pub(super) fn from_boxed(entries: Box<[Bucket<K, V>]>) -> Box<Self> {
        unsafe { Box::from_raw(Box::into_raw(entries) as *mut Self) }
    }
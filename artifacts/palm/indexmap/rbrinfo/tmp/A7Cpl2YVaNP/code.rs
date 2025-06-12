fn into_boxed(self: Box<Self>) -> Box<[Bucket<K, V>]> {
        unsafe { Box::from_raw(Box::into_raw(self) as *mut [Bucket<K, V>]) }
    }
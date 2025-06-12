fn into_boxed(self: Box<Self>) -> Box<[Bucket<T>]> {
        unsafe { Box::from_raw(Box::into_raw(self) as *mut [Bucket<T>]) }
    }
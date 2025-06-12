fn next(&mut self) -> Option<Bucket<T>> {
        unsafe {
            // SAFETY: We set checker flag to true.
            self.next_impl::<true>()
        }
    }
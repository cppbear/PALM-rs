pub fn get(&self) -> Option<&T> {
            // Safe due to `inner`'s invariant of being written to at most once.
            // Had multiple writes to `inner` been allowed, a reference to the
            // value we return now would become dangling by a write of a
            // different value later.
            unsafe { &*self.inner.get() }.as_ref()
        }
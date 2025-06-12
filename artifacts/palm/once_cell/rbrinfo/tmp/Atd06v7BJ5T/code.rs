pub(crate) fn is_initialized(&self) -> bool {
        // An `Acquire` load is enough because that makes all the initialization
        // operations visible to us, and, this being a fast path, weaker
        // ordering helps with performance. This `Acquire` synchronizes with
        // `SeqCst` operations on the slow path.
        self.queue.load(Ordering::Acquire) == COMPLETE_PTR
    }
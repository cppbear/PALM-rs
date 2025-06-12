pub fn wait(&self) -> &T {
            if !self.0.is_initialized() {
                self.0.wait()
            }
            debug_assert!(self.0.is_initialized());
            // Safe b/c of the wait call above and the fact that we didn't
            // relinquish our borrow.
            unsafe { self.get_unchecked() }
        }
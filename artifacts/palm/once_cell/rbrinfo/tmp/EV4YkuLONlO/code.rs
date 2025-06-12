fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            write!(f, "OnceBox({:?})", self.inner.load(Ordering::Relaxed))
        }
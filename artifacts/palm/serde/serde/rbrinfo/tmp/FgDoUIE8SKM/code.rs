fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let mut debug = formatter.debug_tuple("Error");
        #[cfg(any(feature = "std", feature = "alloc"))]
        debug.field(&self.err);
        debug.finish()
    }
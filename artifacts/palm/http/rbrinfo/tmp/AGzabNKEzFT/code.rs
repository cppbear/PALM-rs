fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MaxSizeReached")
            // skip _priv noise
            .finish()
    }
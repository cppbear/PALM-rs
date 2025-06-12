fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("InvalidHeaderName")
            // skip _priv noise
            .finish()
    }
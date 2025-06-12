fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("InvalidHeaderValue")
            // skip _priv noise
            .finish()
    }
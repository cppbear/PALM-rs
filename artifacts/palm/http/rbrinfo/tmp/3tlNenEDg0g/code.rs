fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("InvalidStatusCode")
            // skip _priv noise
            .finish()
    }
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response")
            .field("status", &self.status())
            .field("version", &self.version())
            .field("headers", self.headers())
            // omits Extensions because not useful
            .field("body", self.body())
            .finish()
    }
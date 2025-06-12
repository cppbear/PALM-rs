fn from(path_and_query: PathAndQuery) -> Self {
        Self {
            scheme: Scheme::empty(),
            authority: Authority::empty(),
            path_and_query,
        }
    }
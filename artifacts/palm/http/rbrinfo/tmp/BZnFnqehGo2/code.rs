fn from(authority: Authority) -> Self {
        Self {
            scheme: Scheme::empty(),
            authority,
            path_and_query: PathAndQuery::empty(),
        }
    }
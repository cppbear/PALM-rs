fn default() -> Uri {
        Uri {
            scheme: Scheme::empty(),
            authority: Authority::empty(),
            path_and_query: PathAndQuery::slash(),
        }
    }
pub fn query(&self) -> Option<&str> {
        self.path_and_query.query()
    }
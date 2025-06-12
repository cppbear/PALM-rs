pub fn path(&self) -> &str {
        if self.has_path() {
            self.path_and_query.path()
        } else {
            ""
        }
    }
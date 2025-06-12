fn is_lookaround_prefix(&self) -> bool {
        self.bump_if("?=")
        || self.bump_if("?!")
        || self.bump_if("?<=")
        || self.bump_if("?<!")
    }
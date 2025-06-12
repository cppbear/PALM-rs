pub fn authority(&self) -> Option<&Authority> {
        if self.authority.data.is_empty() {
            None
        } else {
            Some(&self.authority)
        }
    }
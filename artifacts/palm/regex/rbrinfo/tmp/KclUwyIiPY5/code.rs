pub fn is_end(&self) -> bool {
        self.c.is_none() && self.byte.is_none()
    }
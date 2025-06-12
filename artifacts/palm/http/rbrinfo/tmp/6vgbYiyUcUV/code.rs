pub fn is_safe(&self) -> bool {
        matches!(self.0, Get | Head | Options | Trace)
    }
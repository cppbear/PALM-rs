fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        Pointer::fmt(self.0, formatter)
    }
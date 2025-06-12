fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Span({:?}, {:?})", self.start, self.end)
    }
fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Position(o: {:?}, l: {:?}, c: {:?})",
            self.offset, self.line, self.column)
    }
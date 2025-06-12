fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        if self.0 == 1 {
            formatter.write_str("1 element in sequence")
        } else {
            write!(formatter, "{} elements in sequence", self.0)
        }
    }
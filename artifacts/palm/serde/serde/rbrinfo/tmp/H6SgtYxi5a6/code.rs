fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        if self.0 == 1 {
            formatter.write_str("1 element in map")
        } else {
            write!(formatter, "{} elements in map", self.0)
        }
    }
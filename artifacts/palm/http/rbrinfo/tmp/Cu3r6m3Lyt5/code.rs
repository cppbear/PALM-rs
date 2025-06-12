fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.data.is_empty() {
            match self.data.as_bytes()[0] {
                b'/' | b'*' => write!(fmt, "{}", &self.data[..]),
                _ => write!(fmt, "/{}", &self.data[..]),
            }
        } else {
            write!(fmt, "/")
        }
    }
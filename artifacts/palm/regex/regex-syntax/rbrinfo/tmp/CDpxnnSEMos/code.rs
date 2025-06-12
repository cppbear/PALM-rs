fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        ::error::Formatter::from(self).fmt(f)
    }
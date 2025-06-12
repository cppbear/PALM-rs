fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(scheme) = self.scheme() {
            write!(f, "{}://", scheme)?;
        }

        if let Some(authority) = self.authority() {
            write!(f, "{}", authority)?;
        }

        write!(f, "{}", self.path())?;

        if let Some(query) = self.query() {
            write!(f, "?{}", query)?;
        }

        Ok(())
    }
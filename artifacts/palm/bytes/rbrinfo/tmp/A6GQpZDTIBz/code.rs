fn from(error: TryGetError) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, error)
    }
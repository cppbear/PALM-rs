fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DecodeError(e) => write!(f, "DecodeError: {}", e),
            Self::OutputSliceTooSmall => write!(f, "Output slice too small"),
        }
    }
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutputSliceTooSmall => write!(f, "Output slice too small"),
        }
    }
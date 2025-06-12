fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            DecodeSliceError::DecodeError(e) => Some(e),
            DecodeSliceError::OutputSliceTooSmall => None,
        }
    }
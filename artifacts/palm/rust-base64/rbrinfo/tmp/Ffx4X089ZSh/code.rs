fn from(e: DecodeError) -> Self {
        DecodeSliceError::DecodeError(e)
    }
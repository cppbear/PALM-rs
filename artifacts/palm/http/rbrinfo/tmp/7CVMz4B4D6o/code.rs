fn try_from(vec: Vec<u8>) -> Result<Self, Self::Error> {
        Self::from_bytes(&vec)
    }
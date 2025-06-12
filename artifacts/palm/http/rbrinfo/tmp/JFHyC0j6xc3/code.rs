pub(super) fn from_shared(s: Bytes) -> Result<Self, InvalidUri> {
        // Precondition on create_authority: trivially satisfied by the
        // identity closure
        create_authority(s, |s| s)
    }
fn try_from(s: &'a [u8]) -> Result<Self, Self::Error> {
        // parse first, and only turn into Bytes if valid

        // Preconditon on create_authority: copy_from_slice() copies all of
        // bytes from the [u8] parameter into a new Bytes
        create_authority(s, Bytes::copy_from_slice)
    }
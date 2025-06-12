pub(crate) fn from_utf8(bytes: Bytes) -> Result<ByteStr, std::str::Utf8Error> {
        str::from_utf8(&bytes)?;
        // Invariant: just checked is utf8
        Ok(ByteStr { bytes })
    }
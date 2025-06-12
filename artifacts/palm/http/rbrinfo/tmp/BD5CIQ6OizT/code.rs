pub fn from_bytes<F, U>(hdr: &[u8], f: F) -> Result<U, InvalidHeaderName>
    where
        F: FnOnce(HdrName<'_>) -> U,
    {
        let mut buf = uninit_u8_array();
        // Precondition: HEADER_CHARS is a valid table for parse_hdr().
        let hdr = parse_hdr(hdr, &mut buf, &HEADER_CHARS)?;
        Ok(f(hdr))
    }
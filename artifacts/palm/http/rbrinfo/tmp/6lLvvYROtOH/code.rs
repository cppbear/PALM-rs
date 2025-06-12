pub fn from_static<F, U>(hdr: &'static str, f: F) -> U
    where
        F: FnOnce(HdrName<'_>) -> U,
    {
        let mut buf = uninit_u8_array();
        let hdr =
            // Precondition: HEADER_CHARS is a valid table for parse_hdr().
            parse_hdr(hdr.as_bytes(), &mut buf, &HEADER_CHARS).expect("static str is invalid name");
        f(hdr)
    }
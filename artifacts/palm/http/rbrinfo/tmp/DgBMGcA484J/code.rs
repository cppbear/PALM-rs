pub fn from_u16(src: u16) -> Result<StatusCode, InvalidStatusCode> {
        if !(100..1000).contains(&src) {
            return Err(InvalidStatusCode::new());
        }

        NonZeroU16::new(src)
            .map(StatusCode)
            .ok_or_else(InvalidStatusCode::new)
    }
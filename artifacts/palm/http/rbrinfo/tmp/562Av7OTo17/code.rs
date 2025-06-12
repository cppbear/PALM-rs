fn from(hdr: StandardHeader) -> HdrName<'a> {
        HdrName {
            inner: Repr::Standard(hdr),
        }
    }
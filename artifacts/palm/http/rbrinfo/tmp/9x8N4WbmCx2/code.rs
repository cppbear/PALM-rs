fn custom(buf: &'a [u8], lower: bool) -> HdrName<'a> {
        HdrName {
            // Invariant (on MaybeLower): follows from the precondition
            inner: Repr::Custom(MaybeLower { buf, lower }),
        }
    }
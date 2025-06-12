fn from(src: StandardHeader) -> HeaderName {
        HeaderName {
            inner: Repr::Standard(src),
        }
    }
fn from(src: Custom) -> HeaderName {
        HeaderName {
            inner: Repr::Custom(src),
        }
    }
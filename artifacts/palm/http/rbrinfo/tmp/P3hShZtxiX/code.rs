fn from(h: HeaderName) -> HeaderValue {
        HeaderValue {
            inner: h.into_bytes(),
            is_sensitive: false,
        }
    }
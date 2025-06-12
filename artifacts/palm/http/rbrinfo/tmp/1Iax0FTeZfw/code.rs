fn try_from_generic<T: AsRef<[u8]>, F: FnOnce(T) -> Bytes>(
        src: T,
        into: F,
    ) -> Result<HeaderValue, InvalidHeaderValue> {
        for &b in src.as_ref() {
            if !is_valid(b) {
                return Err(InvalidHeaderValue { _priv: () });
            }
        }
        Ok(HeaderValue {
            inner: into(src),
            is_sensitive: false,
        })
    }
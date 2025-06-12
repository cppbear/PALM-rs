pub fn from_maybe_shared<T>(src: T) -> Result<HeaderValue, InvalidHeaderValue>
    where
        T: AsRef<[u8]> + 'static,
    {
        if_downcast_into!(T, Bytes, src, {
            return HeaderValue::from_shared(src);
        });

        HeaderValue::from_bytes(src.as_ref())
    }
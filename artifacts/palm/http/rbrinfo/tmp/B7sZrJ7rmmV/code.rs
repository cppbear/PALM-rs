pub unsafe fn from_maybe_shared_unchecked<T>(src: T) -> HeaderValue
    where
        T: AsRef<[u8]> + 'static,
    {
        if cfg!(debug_assertions) {
            match HeaderValue::from_maybe_shared(src) {
                Ok(val) => val,
                Err(_err) => {
                    panic!("HeaderValue::from_maybe_shared_unchecked() with invalid bytes");
                }
            }
        } else {
            if_downcast_into!(T, Bytes, src, {
                return HeaderValue {
                    inner: src,
                    is_sensitive: false,
                };
            });

            let src = Bytes::copy_from_slice(src.as_ref());
            HeaderValue {
                inner: src,
                is_sensitive: false,
            }
        }
    }
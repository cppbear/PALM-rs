pub const fn from_static(src: &'static str) -> HeaderValue {
        let bytes = src.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            if !is_visible_ascii(bytes[i]) {
                // TODO: When msrv is bumped to larger than 1.57, this should be
                // replaced with `panic!` macro.
                // https://blog.rust-lang.org/2021/12/02/Rust-1.57.0.html#panic-in-const-contexts
                //
                // See the panics section of this method's document for details.
                #[allow(clippy::no_effect, clippy::out_of_bounds_indexing)]
                ([] as [u8; 0])[0]; // Invalid header value
            }
            i += 1;
        }

        HeaderValue {
            inner: Bytes::from_static(bytes),
            is_sensitive: false,
        }
    }
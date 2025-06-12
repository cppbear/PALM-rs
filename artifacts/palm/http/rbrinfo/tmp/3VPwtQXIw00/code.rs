pub const fn from_static(src: &'static str) -> HeaderName {
        let name_bytes = src.as_bytes();
        if let Some(standard) = StandardHeader::from_bytes(name_bytes) {
            return HeaderName {
                inner: Repr::Standard(standard),
            };
        }

        if name_bytes.is_empty() || name_bytes.len() > super::MAX_HEADER_NAME_LEN || {
            let mut i = 0;
            loop {
                if i >= name_bytes.len() {
                    break false;
                } else if HEADER_CHARS_H2[name_bytes[i] as usize] == 0 {
                    break true;
                }
                i += 1;
            }
        } {
            // TODO: When msrv is bumped to larger than 1.57, this should be
            // replaced with `panic!` macro.
            // https://blog.rust-lang.org/2021/12/02/Rust-1.57.0.html#panic-in-const-contexts
            //
            // See the panics section of this method's document for details.
            #[allow(clippy::no_effect, clippy::out_of_bounds_indexing)]
            ([] as [u8; 0])[0]; // Invalid header name
        }

        HeaderName {
            inner: Repr::Custom(Custom(ByteStr::from_static(src))),
        }
    }
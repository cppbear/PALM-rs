fn from(src: HdrName<'a>) -> HeaderName {
        match src.inner {
            Repr::Standard(s) => HeaderName {
                inner: Repr::Standard(s),
            },
            Repr::Custom(maybe_lower) => {
                if maybe_lower.lower {
                    let buf = Bytes::copy_from_slice(maybe_lower.buf);
                    // Safety: the invariant on MaybeLower ensures buf is valid UTF-8.
                    let byte_str = unsafe { ByteStr::from_utf8_unchecked(buf) };

                    HeaderName {
                        inner: Repr::Custom(Custom(byte_str)),
                    }
                } else {
                    use bytes::BufMut;
                    let mut dst = BytesMut::with_capacity(maybe_lower.buf.len());

                    for b in maybe_lower.buf.iter() {
                        // HEADER_CHARS maps each byte to a valid single-byte UTF-8
                        // codepoint.
                        dst.put_u8(HEADER_CHARS[*b as usize]);
                    }

                    // Safety: the loop above maps each byte of maybe_lower.buf to a
                    // valid single-byte UTF-8 codepoint before copying it into dst.
                    // dst (and hence dst.freeze()) is thus valid UTF-8.
                    let buf = unsafe { ByteStr::from_utf8_unchecked(dst.freeze()) };

                    HeaderName {
                        inner: Repr::Custom(Custom(buf)),
                    }
                }
            }
        }
    }
pub fn from_bytes(src: &[u8]) -> Result<HeaderName, InvalidHeaderName> {
        let mut buf = uninit_u8_array();
        // Precondition: HEADER_CHARS is a valid table for parse_hdr().
        match parse_hdr(src, &mut buf, &HEADER_CHARS)?.inner {
            Repr::Standard(std) => Ok(std.into()),
            Repr::Custom(MaybeLower { buf, lower: true }) => {
                let buf = Bytes::copy_from_slice(buf);
                // Safety: the invariant on MaybeLower ensures buf is valid UTF-8.
                let val = unsafe { ByteStr::from_utf8_unchecked(buf) };
                Ok(Custom(val).into())
            }
            Repr::Custom(MaybeLower { buf, lower: false }) => {
                use bytes::BufMut;
                let mut dst = BytesMut::with_capacity(buf.len());

                for b in buf.iter() {
                    // HEADER_CHARS maps all bytes to valid single-byte UTF-8
                    let b = HEADER_CHARS[*b as usize];

                    if b == 0 {
                        return Err(InvalidHeaderName::new());
                    }

                    dst.put_u8(b);
                }

                // Safety: the loop above maps all bytes in buf to valid single byte
                // UTF-8 before copying them into dst. This means that dst (and hence
                // dst.freeze()) is valid UTF-8.
                let val = unsafe { ByteStr::from_utf8_unchecked(dst.freeze()) };

                Ok(Custom(val).into())
            }
        }
    }
pub fn from_lowercase(src: &[u8]) -> Result<HeaderName, InvalidHeaderName> {
        let mut buf = uninit_u8_array();
        // Precondition: HEADER_CHARS_H2 is a valid table for parse_hdr()
        match parse_hdr(src, &mut buf, &HEADER_CHARS_H2)?.inner {
            Repr::Standard(std) => Ok(std.into()),
            Repr::Custom(MaybeLower { buf, lower: true }) => {
                let buf = Bytes::copy_from_slice(buf);
                // Safety: the invariant on MaybeLower ensures buf is valid UTF-8.
                let val = unsafe { ByteStr::from_utf8_unchecked(buf) };
                Ok(Custom(val).into())
            }
            Repr::Custom(MaybeLower { buf, lower: false }) => {
                for &b in buf.iter() {
                    // HEADER_CHARS_H2 maps all bytes that are not valid single-byte
                    // UTF-8 to 0 so this check returns an error for invalid UTF-8.
                    if HEADER_CHARS_H2[b as usize] == 0 {
                        return Err(InvalidHeaderName::new());
                    }
                }

                let buf = Bytes::copy_from_slice(buf);
                // Safety: the loop above checks that each byte of buf (either
                // version) is valid UTF-8.
                let val = unsafe { ByteStr::from_utf8_unchecked(buf) };
                Ok(Custom(val).into())
            }
        }
    }
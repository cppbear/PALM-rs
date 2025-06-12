pub(super) fn from_shared(mut src: Bytes) -> Result<Self, InvalidUri> {
        let mut query = NONE;
        let mut fragment = None;

        let mut is_maybe_not_utf8 = false;

        // block for iterator borrow
        {
            let mut iter = src.as_ref().iter().enumerate();

            // path ...
            for (i, &b) in &mut iter {
                // See https://url.spec.whatwg.org/#path-state
                match b {
                    b'?' => {
                        debug_assert_eq!(query, NONE);
                        query = i as u16;
                        break;
                    }
                    b'#' => {
                        fragment = Some(i);
                        break;
                    }

                    // This is the range of bytes that don't need to be
                    // percent-encoded in the path. If it should have been
                    // percent-encoded, then error.
                    #[rustfmt::skip]
                    0x21 |
                    0x24..=0x3B |
                    0x3D |
                    0x40..=0x5F |
                    0x61..=0x7A |
                    0x7C |
                    0x7E => {}

                    // potentially utf8, might not, should check
                    0x7F..=0xFF => {
                        is_maybe_not_utf8 = true;
                    }

                    // These are code points that are supposed to be
                    // percent-encoded in the path but there are clients
                    // out there sending them as is and httparse accepts
                    // to parse those requests, so they are allowed here
                    // for parity.
                    //
                    // For reference, those are code points that are used
                    // to send requests with JSON directly embedded in
                    // the URI path. Yes, those things happen for real.
                    #[rustfmt::skip]
                    b'"' |
                    b'{' | b'}' => {}

                    _ => return Err(ErrorKind::InvalidUriChar.into()),
                }
            }

            // query ...
            if query != NONE {
                for (i, &b) in iter {
                    match b {
                        // While queries *should* be percent-encoded, most
                        // bytes are actually allowed...
                        // See https://url.spec.whatwg.org/#query-state
                        //
                        // Allowed: 0x21 / 0x24 - 0x3B / 0x3D / 0x3F - 0x7E
                        #[rustfmt::skip]
                        0x21 |
                        0x24..=0x3B |
                        0x3D |
                        0x3F..=0x7E => {}

                        0x7F..=0xFF => {
                            is_maybe_not_utf8 = true;
                        }

                        b'#' => {
                            fragment = Some(i);
                            break;
                        }

                        _ => return Err(ErrorKind::InvalidUriChar.into()),
                    }
                }
            }
        }

        if let Some(i) = fragment {
            src.truncate(i);
        }

        let data = if is_maybe_not_utf8 {
            ByteStr::from_utf8(src).map_err(|_| ErrorKind::InvalidUriChar)?
        } else {
            unsafe { ByteStr::from_utf8_unchecked(src) }
        };

        Ok(PathAndQuery { data, query })
    }
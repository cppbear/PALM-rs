pub(super) fn parse(s: &[u8]) -> Result<usize, InvalidUri> {
        let mut colon_cnt = 0u32;
        let mut start_bracket = false;
        let mut end_bracket = false;
        let mut has_percent = false;
        let mut end = s.len();
        let mut at_sign_pos = None;
        const MAX_COLONS: u32 = 8; // e.g., [FEDC:BA98:7654:3210:FEDC:BA98:7654:3210]:80

        // Among other things, this loop checks that every byte in s up to the
        // first '/', '?', or '#' is a valid URI character (or in some contexts,
        // a '%'). This means that each such byte is a valid single-byte UTF-8
        // code point.
        for (i, &b) in s.iter().enumerate() {
            match URI_CHARS[b as usize] {
                b'/' | b'?' | b'#' => {
                    end = i;
                    break;
                }
                b':' => {
                    if colon_cnt >= MAX_COLONS {
                        return Err(ErrorKind::InvalidAuthority.into());
                    }
                    colon_cnt += 1;
                }
                b'[' => {
                    if has_percent || start_bracket {
                        // Something other than the userinfo has a `%`, so reject it.
                        return Err(ErrorKind::InvalidAuthority.into());
                    }
                    start_bracket = true;
                }
                b']' => {
                    if (!start_bracket) || end_bracket {
                        return Err(ErrorKind::InvalidAuthority.into());
                    }
                    end_bracket = true;

                    // Those were part of an IPv6 hostname, so forget them...
                    colon_cnt = 0;
                    has_percent = false;
                }
                b'@' => {
                    at_sign_pos = Some(i);

                    // Those weren't a port colon, but part of the
                    // userinfo, so it needs to be forgotten.
                    colon_cnt = 0;
                    has_percent = false;
                }
                0 if b == b'%' => {
                    // Per https://tools.ietf.org/html/rfc3986#section-3.2.1 and
                    // https://url.spec.whatwg.org/#authority-state
                    // the userinfo can have a percent-encoded username and password,
                    // so record that a `%` was found. If this turns out to be
                    // part of the userinfo, this flag will be cleared.
                    // Also per https://tools.ietf.org/html/rfc6874, percent-encoding can
                    // be used to indicate a zone identifier.
                    // If the flag hasn't been cleared at the end, that means this
                    // was part of the hostname (and not part of an IPv6 address), and
                    // will fail with an error.
                    has_percent = true;
                }
                0 => {
                    return Err(ErrorKind::InvalidUriChar.into());
                }
                _ => {}
            }
        }

        if start_bracket ^ end_bracket {
            return Err(ErrorKind::InvalidAuthority.into());
        }

        if colon_cnt > 1 {
            // Things like 'localhost:8080:3030' are rejected.
            return Err(ErrorKind::InvalidAuthority.into());
        }

        if end > 0 && at_sign_pos == Some(end - 1) {
            // If there's nothing after an `@`, this is bonkers.
            return Err(ErrorKind::InvalidAuthority.into());
        }

        if has_percent {
            // Something after the userinfo has a `%`, so reject it.
            return Err(ErrorKind::InvalidAuthority.into());
        }

        Ok(end)
    }
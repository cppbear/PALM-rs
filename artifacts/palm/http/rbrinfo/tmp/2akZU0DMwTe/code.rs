pub(super) fn parse(s: &[u8]) -> Result<Scheme2<usize>, InvalidUri> {
        if s.len() >= 7 {
            // Check for HTTP
            if s[..7].eq_ignore_ascii_case(b"http://") {
                // Prefix will be striped
                return Ok(Protocol::Http.into());
            }
        }

        if s.len() >= 8 {
            // Check for HTTPs
            if s[..8].eq_ignore_ascii_case(b"https://") {
                return Ok(Protocol::Https.into());
            }
        }

        if s.len() > 3 {
            for i in 0..s.len() {
                let b = s[i];

                match SCHEME_CHARS[b as usize] {
                    b':' => {
                        // Not enough data remaining
                        if s.len() < i + 3 {
                            break;
                        }

                        // Not a scheme
                        if &s[i + 1..i + 3] != b"//" {
                            break;
                        }

                        if i > MAX_SCHEME_LEN {
                            return Err(ErrorKind::SchemeTooLong.into());
                        }

                        // Return scheme
                        return Ok(Scheme2::Other(i));
                    }
                    // Invalid scheme character, abort
                    0 => break,
                    _ => {}
                }
            }
        }

        Ok(Scheme2::None)
    }
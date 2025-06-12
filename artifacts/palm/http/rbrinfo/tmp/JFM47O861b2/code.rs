fn from_shared(s: Bytes) -> Result<Uri, InvalidUri> {
        use self::ErrorKind::*;

        if s.len() > MAX_LEN {
            return Err(TooLong.into());
        }

        match s.len() {
            0 => {
                return Err(Empty.into());
            }
            1 => match s[0] {
                b'/' => {
                    return Ok(Uri {
                        scheme: Scheme::empty(),
                        authority: Authority::empty(),
                        path_and_query: PathAndQuery::slash(),
                    });
                }
                b'*' => {
                    return Ok(Uri {
                        scheme: Scheme::empty(),
                        authority: Authority::empty(),
                        path_and_query: PathAndQuery::star(),
                    });
                }
                _ => {
                    let authority = Authority::from_shared(s)?;

                    return Ok(Uri {
                        scheme: Scheme::empty(),
                        authority,
                        path_and_query: PathAndQuery::empty(),
                    });
                }
            },
            _ => {}
        }

        if s[0] == b'/' {
            return Ok(Uri {
                scheme: Scheme::empty(),
                authority: Authority::empty(),
                path_and_query: PathAndQuery::from_shared(s)?,
            });
        }

        parse_full(s)
    }
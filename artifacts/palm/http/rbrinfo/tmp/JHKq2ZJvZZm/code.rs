fn try_from(s: &'a [u8]) -> Result<Self, Self::Error> {
        use self::Scheme2::*;

        match Scheme2::parse_exact(s)? {
            None => Err(ErrorKind::InvalidScheme.into()),
            Standard(p) => Ok(Standard(p).into()),
            Other(_) => {
                let bytes = Bytes::copy_from_slice(s);

                // Safety: postcondition on parse_exact() means that s and
                // hence bytes are valid UTF-8.
                let string = unsafe { ByteStr::from_utf8_unchecked(bytes) };

                Ok(Other(Box::new(string)).into())
            }
        }
    }
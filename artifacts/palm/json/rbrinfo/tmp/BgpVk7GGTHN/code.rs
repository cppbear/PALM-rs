fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'a, 's, str>> {
        self.delegate.parse_str_bytes(scratch, true, |_, bytes| {
            // The deserialization input came in as &str with a UTF-8 guarantee,
            // and the \u-escapes are checked along the way, so don't need to
            // check here.
            Ok(unsafe { str::from_utf8_unchecked(bytes) })
        })
    }
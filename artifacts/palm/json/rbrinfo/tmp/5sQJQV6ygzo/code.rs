fn parse_str<'s>(&'s mut self, scratch: &'s mut Vec<u8>) -> Result<Reference<'de, 's, str>> {
        self.parse_str_bytes(scratch, true, as_str)
            .map(Reference::Copied)
    }
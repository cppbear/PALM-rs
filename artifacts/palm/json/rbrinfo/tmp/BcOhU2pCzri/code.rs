fn parse_str_raw<'s>(
        &'s mut self,
        scratch: &'s mut Vec<u8>,
    ) -> Result<Reference<'de, 's, [u8]>> {
        self.parse_str_bytes(scratch, false, |_, bytes| Ok(bytes))
            .map(Reference::Copied)
    }
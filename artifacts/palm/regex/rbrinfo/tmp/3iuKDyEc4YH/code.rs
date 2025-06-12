fn previous_char(&self, at: InputAt) -> Char {
        decode_last_utf8(&self[..at.pos()]).map(|(c, _)| c).into()
    }
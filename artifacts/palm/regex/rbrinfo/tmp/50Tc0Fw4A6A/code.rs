fn next_char(&self, at: InputAt) -> Char {
        decode_utf8(&self[at.pos()..]).map(|(c, _)| c).into()
    }
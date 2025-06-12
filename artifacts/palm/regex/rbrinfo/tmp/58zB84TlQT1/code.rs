pub fn new(text: &'t [u8], only_utf8: bool) -> ByteInput<'t> {
        ByteInput {
            text: text,
            only_utf8: only_utf8,
        }
    }
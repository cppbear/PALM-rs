pub fn new() -> TranslatorBuilder {
        TranslatorBuilder {
            allow_invalid_utf8: false,
            flags: Flags::default(),
        }
    }
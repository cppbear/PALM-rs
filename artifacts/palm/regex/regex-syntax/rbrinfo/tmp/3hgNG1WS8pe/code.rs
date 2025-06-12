pub fn new() -> ParserBuilder {
        ParserBuilder {
            ignore_whitespace: false,
            nest_limit: 250,
            octal: false,
        }
    }
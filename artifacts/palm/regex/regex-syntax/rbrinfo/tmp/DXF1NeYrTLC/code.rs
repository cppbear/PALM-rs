fn new(parser: P, pattern: &'s str) -> ParserI<'s, P> {
        ParserI { parser: parser, pattern: pattern }
    }
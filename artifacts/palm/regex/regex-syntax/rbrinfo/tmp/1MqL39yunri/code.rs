fn new(p: &'p ParserI<'s, P>) -> NestLimiter<'p, 's, P> {
        NestLimiter { p: p, depth: 0 }
    }
fn try_from(r: Range<X>) -> Result<Uniform<X>, Error> {
        Uniform::new(r.start, r.end)
    }
fn try_from(r: ::core::ops::RangeInclusive<X>) -> Result<Uniform<X>, Error> {
        Uniform::new_inclusive(r.start(), r.end())
    }
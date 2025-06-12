fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tuple = f.debug_tuple("RawEntryMut");
        match self {
            Self::Vacant(v) => tuple.field(v),
            Self::Occupied(o) => tuple.field(o),
        };
        tuple.finish()
    }
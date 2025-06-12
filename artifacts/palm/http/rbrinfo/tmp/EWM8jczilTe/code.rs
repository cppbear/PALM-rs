pub fn from_maybe_shared<T>(src: T) -> Result<Self, InvalidUri>
    where
        T: AsRef<[u8]> + 'static,
    {
        if_downcast_into!(T, Bytes, src, {
            return PathAndQuery::from_shared(src);
        });

        PathAndQuery::try_from(src.as_ref())
    }
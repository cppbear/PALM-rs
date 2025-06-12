pub fn from_maybe_shared<T>(src: T) -> Result<Self, InvalidUri>
    where
        T: AsRef<[u8]> + 'static,
    {
        if_downcast_into!(T, Bytes, src, {
            return Authority::from_shared(src);
        });

        Authority::try_from(src.as_ref())
    }
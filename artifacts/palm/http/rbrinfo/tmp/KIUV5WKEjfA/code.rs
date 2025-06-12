pub fn try_entry<K>(&mut self, key: K) -> Result<Entry<'_, T>, InvalidHeaderName>
    where
        K: AsHeaderName,
    {
        key.try_entry(self).map_err(|err| match err {
            as_header_name::TryEntryError::InvalidHeaderName(e) => e,
            as_header_name::TryEntryError::MaxSizeReached(_e) => {
                // Unfortunately, we cannot change the return type of this
                // method, so the max size reached error needs to be converted
                // into an InvalidHeaderName. Yay.
                InvalidHeaderName::new()
            }
        })
    }
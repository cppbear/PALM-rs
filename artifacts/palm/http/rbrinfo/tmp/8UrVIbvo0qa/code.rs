fn try_from(c: &'a HashMap<K, V, S>) -> Result<Self, Self::Error> {
        c.iter()
            .map(|(k, v)| -> crate::Result<(HeaderName, T)> {
                let name = TryFrom::try_from(k).map_err(Into::into)?;
                let value = TryFrom::try_from(v).map_err(Into::into)?;
                Ok((name, value))
            })
            .collect()
    }
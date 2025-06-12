fn deserialize_other<V>() -> Result<V, E> {
        Err(Error::custom("can only flatten structs and maps"))
    }
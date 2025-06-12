fn bad_type(what: Unsupported) -> M::Error {
        ser::Error::custom(format_args!(
            "can only flatten structs and maps (got {})",
            what
        ))
    }
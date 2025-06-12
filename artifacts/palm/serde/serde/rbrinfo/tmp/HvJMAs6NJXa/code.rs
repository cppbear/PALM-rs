fn visit_unit<E>(self) -> Result<(), E>
        where
            E: de::Error,
        {
            Ok(())
        }
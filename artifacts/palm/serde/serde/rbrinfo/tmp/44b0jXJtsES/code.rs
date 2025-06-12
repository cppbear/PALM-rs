fn visit_none<E>(self) -> Result<(), E>
        where
            E: de::Error,
        {
            Ok(())
        }
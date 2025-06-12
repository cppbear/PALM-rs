fn visit_seq<S>(self, _: S) -> Result<(), S::Error>
        where
            S: SeqAccess<'de>,
        {
            Ok(())
        }
fn visit_map<M>(self, mut access: M) -> Result<(), M::Error>
        where
            M: MapAccess<'de>,
        {
            while tri!(access.next_entry::<IgnoredAny, IgnoredAny>()).is_some() {}
            Ok(())
        }
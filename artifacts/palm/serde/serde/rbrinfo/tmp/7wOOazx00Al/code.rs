fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut end: Option<Idx> = None;
            while let Some(key) = tri!(map.next_key()) {
                match key {
                    Field::End => {
                        if end.is_some() {
                            return Err(<A::Error as Error>::duplicate_field("end"));
                        }
                        end = Some(tri!(map.next_value()));
                    }
                }
            }
            let end = match end {
                Some(end) => end,
                None => return Err(<A::Error as Error>::missing_field("end")),
            };
            Ok(end)
        }
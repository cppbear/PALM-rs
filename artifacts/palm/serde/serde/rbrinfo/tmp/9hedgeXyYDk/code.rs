fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut start: Option<Idx> = None;
            while let Some(key) = tri!(map.next_key()) {
                match key {
                    Field::Start => {
                        if start.is_some() {
                            return Err(<A::Error as Error>::duplicate_field("start"));
                        }
                        start = Some(tri!(map.next_value()));
                    }
                }
            }
            let start = match start {
                Some(start) => start,
                None => return Err(<A::Error as Error>::missing_field("start")),
            };
            Ok(start)
        }
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // If this were outside of the serde crate, it would just use:
        //
        //    #[derive(Deserialize)]
        //    #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Secs,
            Nanos,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`secs` or `nanos`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                    where
                        E: Error,
                    {
                        match value {
                            "secs" => Ok(Field::Secs),
                            "nanos" => Ok(Field::Nanos),
                            _ => Err(Error::unknown_field(value, FIELDS)),
                        }
                    }

                    fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
                    where
                        E: Error,
                    {
                        match value {
                            b"secs" => Ok(Field::Secs),
                            b"nanos" => Ok(Field::Nanos),
                            _ => {
                                let value = crate::__private::from_utf8_lossy(value);
                                Err(Error::unknown_field(&*value, FIELDS))
                            }
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        fn check_overflow<E>(secs: u64, nanos: u32) -> Result<(), E>
        where
            E: Error,
        {
            static NANOS_PER_SEC: u32 = 1_000_000_000;
            match secs.checked_add((nanos / NANOS_PER_SEC) as u64) {
                Some(_) => Ok(()),
                None => Err(E::custom("overflow deserializing Duration")),
            }
        }

        struct DurationVisitor;

        impl<'de> Visitor<'de> for DurationVisitor {
            type Value = Duration;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Duration")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let secs: u64 = match tri!(seq.next_element()) {
                    Some(value) => value,
                    None => {
                        return Err(Error::invalid_length(0, &self));
                    }
                };
                let nanos: u32 = match tri!(seq.next_element()) {
                    Some(value) => value,
                    None => {
                        return Err(Error::invalid_length(1, &self));
                    }
                };
                tri!(check_overflow(secs, nanos));
                Ok(Duration::new(secs, nanos))
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut secs: Option<u64> = None;
                let mut nanos: Option<u32> = None;
                while let Some(key) = tri!(map.next_key()) {
                    match key {
                        Field::Secs => {
                            if secs.is_some() {
                                return Err(<A::Error as Error>::duplicate_field("secs"));
                            }
                            secs = Some(tri!(map.next_value()));
                        }
                        Field::Nanos => {
                            if nanos.is_some() {
                                return Err(<A::Error as Error>::duplicate_field("nanos"));
                            }
                            nanos = Some(tri!(map.next_value()));
                        }
                    }
                }
                let secs = match secs {
                    Some(secs) => secs,
                    None => return Err(<A::Error as Error>::missing_field("secs")),
                };
                let nanos = match nanos {
                    Some(nanos) => nanos,
                    None => return Err(<A::Error as Error>::missing_field("nanos")),
                };
                tri!(check_overflow(secs, nanos));
                Ok(Duration::new(secs, nanos))
            }
        }

        const FIELDS: &[&str] = &["secs", "nanos"];
        deserializer.deserialize_struct("Duration", FIELDS, DurationVisitor)
    }
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct FieldVisitor;

            impl<'de> Visitor<'de> for FieldVisitor {
                type Value = Field;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    formatter.write_str("`start`")
                }

                fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    match value {
                        "start" => Ok(Field::Start),
                        _ => Err(Error::unknown_field(value, FIELDS)),
                    }
                }

                fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    match value {
                        b"start" => Ok(Field::Start),
                        _ => {
                            let value = crate::__private::from_utf8_lossy(value);
                            Err(Error::unknown_field(&*value, FIELDS))
                        }
                    }
                }
            }

            deserializer.deserialize_identifier(FieldVisitor)
        }
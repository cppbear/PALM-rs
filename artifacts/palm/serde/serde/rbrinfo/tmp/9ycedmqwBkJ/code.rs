fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // If this were outside of the serde crate, it would just use:
        //
        //    #[derive(Deserialize)]
        //    #[serde(variant_identifier)]
        enum Field {
            Ok,
            Err,
        }

        impl<'de> Deserialize<'de> for Field {
            #[inline]
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`Ok` or `Err`")
                    }

                    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
                    where
                        E: Error,
                    {
                        match value {
                            0 => Ok(Field::Ok),
                            1 => Ok(Field::Err),
                            _ => Err(Error::invalid_value(Unexpected::Unsigned(value), &self)),
                        }
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                    where
                        E: Error,
                    {
                        match value {
                            "Ok" => Ok(Field::Ok),
                            "Err" => Ok(Field::Err),
                            _ => Err(Error::unknown_variant(value, VARIANTS)),
                        }
                    }

                    fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
                    where
                        E: Error,
                    {
                        match value {
                            b"Ok" => Ok(Field::Ok),
                            b"Err" => Ok(Field::Err),
                            _ => match str::from_utf8(value) {
                                Ok(value) => Err(Error::unknown_variant(value, VARIANTS)),
                                Err(_) => {
                                    Err(Error::invalid_value(Unexpected::Bytes(value), &self))
                                }
                            },
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct ResultVisitor<T, E>(PhantomData<Result<T, E>>);

        impl<'de, T, E> Visitor<'de> for ResultVisitor<T, E>
        where
            T: Deserialize<'de>,
            E: Deserialize<'de>,
        {
            type Value = Result<T, E>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("enum Result")
            }

            fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
            where
                A: EnumAccess<'de>,
            {
                match tri!(data.variant()) {
                    (Field::Ok, v) => v.newtype_variant().map(Ok),
                    (Field::Err, v) => v.newtype_variant().map(Err),
                }
            }
        }

        const VARIANTS: &[&str] = &["Ok", "Err"];

        deserializer.deserialize_enum("Result", VARIANTS, ResultVisitor(PhantomData))
    }
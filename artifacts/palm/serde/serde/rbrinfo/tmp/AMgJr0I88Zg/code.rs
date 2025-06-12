fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Unbounded,
            Included,
            Excluded,
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
                        formatter.write_str("`Unbounded`, `Included` or `Excluded`")
                    }

                    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
                    where
                        E: Error,
                    {
                        match value {
                            0 => Ok(Field::Unbounded),
                            1 => Ok(Field::Included),
                            2 => Ok(Field::Excluded),
                            _ => Err(Error::invalid_value(Unexpected::Unsigned(value), &self)),
                        }
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                    where
                        E: Error,
                    {
                        match value {
                            "Unbounded" => Ok(Field::Unbounded),
                            "Included" => Ok(Field::Included),
                            "Excluded" => Ok(Field::Excluded),
                            _ => Err(Error::unknown_variant(value, VARIANTS)),
                        }
                    }

                    fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
                    where
                        E: Error,
                    {
                        match value {
                            b"Unbounded" => Ok(Field::Unbounded),
                            b"Included" => Ok(Field::Included),
                            b"Excluded" => Ok(Field::Excluded),
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

        struct BoundVisitor<T>(PhantomData<Bound<T>>);

        impl<'de, T> Visitor<'de> for BoundVisitor<T>
        where
            T: Deserialize<'de>,
        {
            type Value = Bound<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("enum Bound")
            }

            fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
            where
                A: EnumAccess<'de>,
            {
                match tri!(data.variant()) {
                    (Field::Unbounded, v) => v.unit_variant().map(|()| Bound::Unbounded),
                    (Field::Included, v) => v.newtype_variant().map(Bound::Included),
                    (Field::Excluded, v) => v.newtype_variant().map(Bound::Excluded),
                }
            }
        }

        const VARIANTS: &[&str] = &["Unbounded", "Included", "Excluded"];

        deserializer.deserialize_enum("Bound", VARIANTS, BoundVisitor(PhantomData))
    }
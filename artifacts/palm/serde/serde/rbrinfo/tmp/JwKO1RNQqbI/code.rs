fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_enum(
            self.enum_name,
            self.variants,
            AdjacentlyTaggedEnumVariantVisitor {
                enum_name: self.enum_name,
                fields_enum: PhantomData,
            },
        )
    }
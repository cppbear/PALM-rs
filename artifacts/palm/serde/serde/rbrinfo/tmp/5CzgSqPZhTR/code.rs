fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
        where
            V: de::DeserializeSeed<'de>,
        {
            let visitor = VariantRefDeserializer {
                value: self.value,
                err: PhantomData,
            };
            seed.deserialize(ContentRefDeserializer::new(self.variant))
                .map(|v| (v, visitor))
        }
fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), E>
        where
            V: de::DeserializeSeed<'de>,
        {
            let visitor = VariantDeserializer {
                value: self.value,
                err: PhantomData,
            };
            seed.deserialize(ContentDeserializer::new(self.variant))
                .map(|v| (v, visitor))
        }
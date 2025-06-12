fn variant<V>(self) -> Result<(V, Self::Variant), Self::Error>
    where
        V: Deserialize<'de>,
    {
        self.variant_seed(PhantomData)
    }
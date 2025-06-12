fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: EnumAccess<'de>,
    {
        let (variant, variant_access) = tri!(data.variant());
        tri!(variant_access.unit_variant());
        Ok(variant)
    }
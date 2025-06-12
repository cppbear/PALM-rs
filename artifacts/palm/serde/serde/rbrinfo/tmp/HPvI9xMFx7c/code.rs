fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: EnumAccess<'de>,
    {
        tri!(data.variant::<IgnoredAny>()).1.newtype_variant()
    }
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_unit_variant(self.enum_name, self.variant_index, self.variant_name)
    }
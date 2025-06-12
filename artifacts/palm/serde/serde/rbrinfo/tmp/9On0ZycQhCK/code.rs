fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Bound::Unbounded => serializer.serialize_unit_variant("Bound", 0, "Unbounded"),
            Bound::Included(ref value) => {
                serializer.serialize_newtype_variant("Bound", 1, "Included", value)
            }
            Bound::Excluded(ref value) => {
                serializer.serialize_newtype_variant("Bound", 2, "Excluded", value)
            }
        }
    }
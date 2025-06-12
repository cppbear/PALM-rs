fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            // Untagged and internally tagged enums are only supported in
            // self-describing formats.
            let visitor = ContentVisitor { value: PhantomData };
            deserializer.__deserialize_content(actually_private::T, visitor)
        }
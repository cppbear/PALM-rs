fn serialize_newtype_variant<T>(
            self,
            name: &'static str,
            variant_index: u32,
            variant: &'static str,
            value: &T,
        ) -> Result<Content, E>
        where
            T: ?Sized + Serialize,
        {
            Ok(Content::NewtypeVariant(
                name,
                variant_index,
                variant,
                Box::new(tri!(value.serialize(self))),
            ))
        }
fn serialize_newtype_struct<T>(self, name: &'static str, value: &T) -> Result<Content, E>
        where
            T: ?Sized + Serialize,
        {
            Ok(Content::NewtypeStruct(
                name,
                Box::new(tri!(value.serialize(self))),
            ))
        }
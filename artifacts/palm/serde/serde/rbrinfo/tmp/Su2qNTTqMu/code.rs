fn serialize_some<T>(self, value: &T) -> Result<Content, E>
        where
            T: ?Sized + Serialize,
        {
            Ok(Content::Some(Box::new(tri!(value.serialize(self)))))
        }
fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Error>
    where
        V: de::Visitor<'de>,
    {
        match self.value {
            Cow::Borrowed(string) => visitor.visit_borrowed_str(string),
            #[cfg(any(feature = "std", feature = "alloc"))]
            Cow::Owned(string) => visitor.visit_string(string),
            #[cfg(not(any(feature = "std", feature = "alloc")))]
            Cow::Owned(_) => unreachable!(),
        }
    }
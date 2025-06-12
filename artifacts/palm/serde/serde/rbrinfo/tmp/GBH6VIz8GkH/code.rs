fn __deserialize_content<V>(
            self,
            _: actually_private::T,
            visitor: V,
        ) -> Result<Content<'de>, Self::Error>
        where
            V: Visitor<'de, Value = Content<'de>>,
        {
            let _ = visitor;
            Ok(self.content.clone())
        }
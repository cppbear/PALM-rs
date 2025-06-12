fn end(self) -> Result<Content, E> {
            Ok(Content::Map(self.entries))
        }
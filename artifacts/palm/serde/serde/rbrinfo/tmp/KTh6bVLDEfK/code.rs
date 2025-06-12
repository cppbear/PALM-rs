fn end(self) -> Result<Content, E> {
            Ok(Content::Struct(self.name, self.fields))
        }
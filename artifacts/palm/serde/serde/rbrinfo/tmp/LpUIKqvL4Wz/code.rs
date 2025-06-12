fn end(self) -> Result<Content, E> {
            Ok(Content::Tuple(self.elements))
        }
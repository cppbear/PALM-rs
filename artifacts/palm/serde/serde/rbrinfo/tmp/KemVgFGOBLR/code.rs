fn end(self) -> Result<Content, E> {
            Ok(Content::Seq(self.elements))
        }
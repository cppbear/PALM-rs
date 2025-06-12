fn end(self) -> Result<Content, E> {
            Ok(Content::TupleStruct(self.name, self.fields))
        }
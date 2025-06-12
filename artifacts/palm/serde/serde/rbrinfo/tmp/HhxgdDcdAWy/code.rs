fn end(self) -> Result<(), Self::Error> {
        tri!(self
            .map
            .serialize_value(&Content::Struct(self.name, self.fields)));
        Ok(())
    }
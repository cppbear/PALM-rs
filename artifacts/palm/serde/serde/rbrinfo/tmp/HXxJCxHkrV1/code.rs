fn end(self) -> Result<(), Self::Error> {
        tri!(self.map.serialize_value(&Content::Seq(self.fields)));
        Ok(())
    }
fn end(mut self) -> Result<M::Ok, M::Error> {
            tri!(self
                .map
                .serialize_value(&Content::Struct(self.name, self.fields)));
            self.map.end()
        }
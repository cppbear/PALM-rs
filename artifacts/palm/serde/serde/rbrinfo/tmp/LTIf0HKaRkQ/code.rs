fn end(mut self) -> Result<M::Ok, M::Error> {
            tri!(self
                .map
                .serialize_value(&Content::TupleStruct(self.name, self.fields)));
            self.map.end()
        }
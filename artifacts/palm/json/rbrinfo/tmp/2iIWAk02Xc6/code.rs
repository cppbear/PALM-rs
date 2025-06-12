fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        tri!(self
            .formatter
            .begin_array(&mut self.writer)
            .map_err(Error::io));
        if len == Some(0) {
            tri!(self
                .formatter
                .end_array(&mut self.writer)
                .map_err(Error::io));
            Ok(Compound::Map {
                ser: self,
                state: State::Empty,
            })
        } else {
            Ok(Compound::Map {
                ser: self,
                state: State::First,
            })
        }
    }
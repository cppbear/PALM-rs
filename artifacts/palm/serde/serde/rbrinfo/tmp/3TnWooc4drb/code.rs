fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        let mut state = tri!(self.delegate.serialize_struct(name, len + 1));
        tri!(state.serialize_field(self.tag, self.variant_name));
        Ok(state)
    }
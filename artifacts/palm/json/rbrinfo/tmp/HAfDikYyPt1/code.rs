fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }
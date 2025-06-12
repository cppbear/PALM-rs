fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Char))
    }
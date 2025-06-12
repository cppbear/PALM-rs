fn serialize_char(self, _: char) -> Result<Self::Ok, Self::Error> {
        Err(Self::bad_type(Unsupported::Char))
    }
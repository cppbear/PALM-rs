fn serialize_u16(self, _: u16) -> Result<Self::Ok, Self::Error> {
        Err(Self::bad_type(Unsupported::Integer))
    }
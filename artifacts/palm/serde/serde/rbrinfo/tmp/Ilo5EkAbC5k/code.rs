fn serialize_u32(self, _: u32) -> Result<Self::Ok, Self::Error> {
        Err(Self::bad_type(Unsupported::Integer))
    }
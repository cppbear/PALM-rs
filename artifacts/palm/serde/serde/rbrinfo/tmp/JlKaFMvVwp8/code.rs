fn serialize_u64(self, _: u64) -> Result<Self::Ok, Self::Error> {
        Err(Self::bad_type(Unsupported::Integer))
    }
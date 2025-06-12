fn serialize_i32(self, _: i32) -> Result<Self::Ok, Self::Error> {
        Err(Self::bad_type(Unsupported::Integer))
    }
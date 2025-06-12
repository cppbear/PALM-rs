fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> {
        Err(Self::bad_type(Unsupported::String))
    }
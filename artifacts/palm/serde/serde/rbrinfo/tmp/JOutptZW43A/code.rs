fn serialize_str(self, _: &str) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::String))
    }
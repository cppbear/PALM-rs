fn serialize_bool(self, _: bool) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Boolean))
    }
fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::Optional))
    }
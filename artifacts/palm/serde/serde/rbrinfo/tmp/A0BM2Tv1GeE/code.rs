fn serialize_bytes(self, _: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(self.bad_type(Unsupported::ByteArray))
    }
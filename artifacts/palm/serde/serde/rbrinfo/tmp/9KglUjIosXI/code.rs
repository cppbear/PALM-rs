fn serialize_i128(self, v: i128) -> Result<Self::Ok, Self::Error> {
        let _ = v;
        Err(Error::custom("i128 is not supported"))
    }
fn serialize_u128(self, v: u128) -> Result<Self::Ok, Self::Error> {
        let _ = v;
        Err(Error::custom("u128 is not supported"))
    }
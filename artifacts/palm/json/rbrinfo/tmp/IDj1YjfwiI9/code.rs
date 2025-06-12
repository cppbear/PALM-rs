fn invalid_type(self, exp: &dyn Expected) -> Error {
        match self {
            ParserNumber::F64(x) => de::Error::invalid_type(Unexpected::Float(x), exp),
            ParserNumber::U64(x) => de::Error::invalid_type(Unexpected::Unsigned(x), exp),
            ParserNumber::I64(x) => de::Error::invalid_type(Unexpected::Signed(x), exp),
            #[cfg(feature = "arbitrary_precision")]
            ParserNumber::String(_) => de::Error::invalid_type(Unexpected::Other("number"), exp),
        }
    }
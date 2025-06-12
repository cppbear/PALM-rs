pub fn borrow_cow_str<'de: 'a, 'a, D, R>(deserializer: D) -> Result<R, D::Error>
where
    D: Deserializer<'de>,
    R: From<Cow<'a, str>>,
{
    struct CowStrVisitor;

    impl<'a> Visitor<'a> for CowStrVisitor {
        type Value = Cow<'a, str>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(Cow::Owned(v.to_owned()))
        }

        fn visit_borrowed_str<E>(self, v: &'a str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(Cow::Borrowed(v))
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(Cow::Owned(v))
        }

        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: Error,
        {
            match str::from_utf8(v) {
                Ok(s) => Ok(Cow::Owned(s.to_owned())),
                Err(_) => Err(Error::invalid_value(Unexpected::Bytes(v), &self)),
            }
        }

        fn visit_borrowed_bytes<E>(self, v: &'a [u8]) -> Result<Self::Value, E>
        where
            E: Error,
        {
            match str::from_utf8(v) {
                Ok(s) => Ok(Cow::Borrowed(s)),
                Err(_) => Err(Error::invalid_value(Unexpected::Bytes(v), &self)),
            }
        }

        fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
        where
            E: Error,
        {
            match String::from_utf8(v) {
                Ok(s) => Ok(Cow::Owned(s)),
                Err(e) => Err(Error::invalid_value(
                    Unexpected::Bytes(&e.into_bytes()),
                    &self,
                )),
            }
        }
    }

    deserializer.deserialize_str(CowStrVisitor).map(From::from)
}
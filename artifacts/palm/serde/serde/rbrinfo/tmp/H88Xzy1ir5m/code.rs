fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: EnumAccess<'de>,
    {
        use std::os::unix::ffi::OsStringExt;

        match tri!(data.variant()) {
            (OsStringKind::Unix, v) => v.newtype_variant().map(OsString::from_vec),
            (OsStringKind::Windows, _) => Err(Error::custom(
                "cannot deserialize Windows OS string on Unix",
            )),
        }
    }
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use std::os::unix::ffi::OsStrExt;
        serializer.serialize_newtype_variant("OsString", 0, "Unix", self.as_bytes())
    }
fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            match *self {
                net::IpAddr::V4(ref a) => a.serialize(serializer),
                net::IpAddr::V6(ref a) => a.serialize(serializer),
            }
        } else {
            match *self {
                net::IpAddr::V4(ref a) => {
                    serializer.serialize_newtype_variant("IpAddr", 0, "V4", a)
                }
                net::IpAddr::V6(ref a) => {
                    serializer.serialize_newtype_variant("IpAddr", 1, "V6", a)
                }
            }
        }
    }
fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserializer.deserialize_str(FromStrVisitor::new("socket address"))
        } else {
            use crate::lib::net::SocketAddr;
            deserialize_enum! {
                SocketAddr SocketAddrKind (V4; b"V4"; 0, V6; b"V6"; 1)
                "`V4` or `V6`",
                deserializer
            }
        }
    }
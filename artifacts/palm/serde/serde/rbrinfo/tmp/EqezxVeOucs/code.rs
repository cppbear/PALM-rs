pub fn as_str(&self) -> Option<&str> {
            match *self {
                Content::Str(x) => Some(x),
                Content::String(ref x) => Some(x),
                Content::Bytes(x) => str::from_utf8(x).ok(),
                Content::ByteBuf(ref x) => str::from_utf8(x).ok(),
                _ => None,
            }
        }
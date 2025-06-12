use std::convert::TryFrom;
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use std::{cmp, fmt, str};
use bytes::Bytes;
use super::{ErrorKind, InvalidUri, Port, URI_CHARS};
use crate::byte_str::ByteStr;
fn host(auth: &str) -> &str {
    let host_port = auth.rsplit('@').next().expect("split always has at least 1 item");
    if host_port.as_bytes()[0] == b'[' {
        let i = host_port.find(']').expect("parsing should validate brackets");
        &host_port[0..i + 1]
    } else {
        host_port.split(':').next().expect("split always has at least 1 item")
    }
}

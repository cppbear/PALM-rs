use super::InvalidMethod;
use std::str;
#[rustfmt::skip]
const METHOD_CHARS: [u8; 256] = [
    b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0',
    b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0',
    b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'!', b'\0', b'#',
    b'$', b'%', b'&', b'\'', b'\0', b'\0', b'*', b'+', b'\0', b'-', b'.', b'\0', b'0',
    b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'\0', b'\0', b'\0', b'\0',
    b'\0', b'\0', b'\0', b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J',
    b'K', b'L', b'M', b'N', b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X',
    b'Y', b'Z', b'\0', b'\0', b'\0', b'^', b'_', b'`', b'a', b'b', b'c', b'd', b'e',
    b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's',
    b't', b'u', b'v', b'w', b'x', b'y', b'z', b'\0', b'|', b'\0', b'~', b'\0', b'\0',
    b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0',
    b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0',
    b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0',
    b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0',
    b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0',
    b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0',
    b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0',
    b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0',
    b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0',
    b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0',
    b'\0', b'\0', b'\0', b'\0', b'\0', b'\0', b'\0',
];
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct AllocatedExtension(Box<[u8]>);
impl AllocatedExtension {
    pub fn new(src: &[u8]) -> Result<AllocatedExtension, InvalidMethod> {}
    pub fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(&self.0) }
    }
}

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
pub struct InlineExtension([u8; InlineExtension::MAX], u8);
pub struct InvalidMethod {
    _priv: (),
}
impl InlineExtension {
    pub const MAX: usize = 15;
    pub fn new(src: &[u8]) -> Result<InlineExtension, InvalidMethod> {
        let mut data: [u8; InlineExtension::MAX] = Default::default();
        write_checked(src, &mut data)?;
        Ok(InlineExtension(data, src.len() as u8))
    }
    pub fn as_str(&self) -> &str {}
}
fn write_checked(src: &[u8], dst: &mut [u8]) -> Result<(), InvalidMethod> {
    for (i, &b) in src.iter().enumerate() {
        let b = METHOD_CHARS[b as usize];
        if b == 0 {
            return Err(InvalidMethod::new());
        }
        dst[i] = b;
    }
    Ok(())
}

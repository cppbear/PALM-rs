use self::extension::{AllocatedExtension, InlineExtension};
use self::Inner::*;
use std::convert::TryFrom;
use std::error::Error;
use std::str::FromStr;
use std::{fmt, str};
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Method(Inner);
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct InlineExtension([u8; InlineExtension::MAX], u8);
pub struct InvalidMethod {
    _priv: (),
}
#[derive(Clone, PartialEq, Eq, Hash)]
enum Inner {
    Options,
    Get,
    Post,
    Put,
    Delete,
    Head,
    Trace,
    Connect,
    Patch,
    ExtensionInline(InlineExtension),
    ExtensionAllocated(AllocatedExtension),
}
impl Method {
    pub const GET: Method = Method(Get);
    pub const POST: Method = Method(Post);
    pub const PUT: Method = Method(Put);
    pub const DELETE: Method = Method(Delete);
    pub const HEAD: Method = Method(Head);
    pub const OPTIONS: Method = Method(Options);
    pub const CONNECT: Method = Method(Connect);
    pub const PATCH: Method = Method(Patch);
    pub const TRACE: Method = Method(Trace);
    pub fn from_bytes(src: &[u8]) -> Result<Method, InvalidMethod> {}
    fn extension_inline(src: &[u8]) -> Result<Method, InvalidMethod> {
        let inline = InlineExtension::new(src)?;
        Ok(Method(ExtensionInline(inline)))
    }
    pub fn is_safe(&self) -> bool {}
    pub fn is_idempotent(&self) -> bool {}
    #[inline]
    pub fn as_str(&self) -> &str {}
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

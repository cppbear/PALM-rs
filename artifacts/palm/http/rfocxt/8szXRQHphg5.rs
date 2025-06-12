use self::extension::{AllocatedExtension, InlineExtension};
use self::Inner::*;
use std::convert::TryFrom;
use std::error::Error;
use std::str::FromStr;
use std::{fmt, str};
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Method(Inner);
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
    fn extension_inline(src: &[u8]) -> Result<Method, InvalidMethod> {}
    pub fn is_safe(&self) -> bool {
        matches!(self.0, Get | Head | Options | Trace)
    }
    pub fn is_idempotent(&self) -> bool {}
    #[inline]
    pub fn as_str(&self) -> &str {}
}

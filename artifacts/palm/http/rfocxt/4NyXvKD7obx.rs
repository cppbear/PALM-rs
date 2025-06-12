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
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct AllocatedExtension(Box<[u8]>);
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
    pub fn is_safe(&self) -> bool {}
    pub fn is_idempotent(&self) -> bool {}
    #[inline]
    pub fn as_str(&self) -> &str {
        match self.0 {
            Options => "OPTIONS",
            Get => "GET",
            Post => "POST",
            Put => "PUT",
            Delete => "DELETE",
            Head => "HEAD",
            Trace => "TRACE",
            Connect => "CONNECT",
            Patch => "PATCH",
            ExtensionInline(ref inline) => inline.as_str(),
            ExtensionAllocated(ref allocated) => allocated.as_str(),
        }
    }
}
impl InlineExtension {
    pub const MAX: usize = 15;
    pub fn new(src: &[u8]) -> Result<InlineExtension, InvalidMethod> {}
    pub fn as_str(&self) -> &str {
        let InlineExtension(ref data, len) = self;
        unsafe { str::from_utf8_unchecked(&data[..*len as usize]) }
    }
}
impl AllocatedExtension {
    pub fn new(src: &[u8]) -> Result<AllocatedExtension, InvalidMethod> {}
    pub fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(&self.0) }
    }
}

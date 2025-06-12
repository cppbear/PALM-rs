use self::extension::{AllocatedExtension, InlineExtension};
use self::Inner::*;
use std::convert::TryFrom;
use std::error::Error;
use std::str::FromStr;
use std::{fmt, str};
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Method(Inner);
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
impl<'a> TryFrom<&'a str> for Method {
    type Error = InvalidMethod;
    #[inline]
    fn try_from(t: &'a str) -> Result<Self, Self::Error> {
        TryFrom::try_from(t.as_bytes())
    }
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
    pub fn from_bytes(src: &[u8]) -> Result<Method, InvalidMethod> {
        match src.len() {
            0 => Err(InvalidMethod::new()),
            3 => {
                match src {
                    b"GET" => Ok(Method(Get)),
                    b"PUT" => Ok(Method(Put)),
                    _ => Method::extension_inline(src),
                }
            }
            4 => {
                match src {
                    b"POST" => Ok(Method(Post)),
                    b"HEAD" => Ok(Method(Head)),
                    _ => Method::extension_inline(src),
                }
            }
            5 => {
                match src {
                    b"PATCH" => Ok(Method(Patch)),
                    b"TRACE" => Ok(Method(Trace)),
                    _ => Method::extension_inline(src),
                }
            }
            6 => {
                match src {
                    b"DELETE" => Ok(Method(Delete)),
                    _ => Method::extension_inline(src),
                }
            }
            7 => {
                match src {
                    b"OPTIONS" => Ok(Method(Options)),
                    b"CONNECT" => Ok(Method(Connect)),
                    _ => Method::extension_inline(src),
                }
            }
            _ => {
                if src.len() <= InlineExtension::MAX {
                    Method::extension_inline(src)
                } else {
                    let allocated = AllocatedExtension::new(src)?;
                    Ok(Method(ExtensionAllocated(allocated)))
                }
            }
        }
    }
    fn extension_inline(src: &[u8]) -> Result<Method, InvalidMethod> {}
    pub fn is_safe(&self) -> bool {}
    pub fn is_idempotent(&self) -> bool {}
    #[inline]
    pub fn as_str(&self) -> &str {}
}

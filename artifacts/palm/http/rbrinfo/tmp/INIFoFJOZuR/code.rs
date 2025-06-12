pub fn from_bytes(src: &[u8]) -> Result<Method, InvalidMethod> {
        match src.len() {
            0 => Err(InvalidMethod::new()),
            3 => match src {
                b"GET" => Ok(Method(Get)),
                b"PUT" => Ok(Method(Put)),
                _ => Method::extension_inline(src),
            },
            4 => match src {
                b"POST" => Ok(Method(Post)),
                b"HEAD" => Ok(Method(Head)),
                _ => Method::extension_inline(src),
            },
            5 => match src {
                b"PATCH" => Ok(Method(Patch)),
                b"TRACE" => Ok(Method(Trace)),
                _ => Method::extension_inline(src),
            },
            6 => match src {
                b"DELETE" => Ok(Method(Delete)),
                _ => Method::extension_inline(src),
            },
            7 => match src {
                b"OPTIONS" => Ok(Method(Options)),
                b"CONNECT" => Ok(Method(Connect)),
                _ => Method::extension_inline(src),
            },
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
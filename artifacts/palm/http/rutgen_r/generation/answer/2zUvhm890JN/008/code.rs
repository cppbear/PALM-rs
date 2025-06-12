// Answer 0

#[test]
fn test_as_str_put() {
    struct HttpMethod(Put);

    impl HttpMethod {
        pub fn as_str(&self) -> &str {
            match self.0 {
                Put => "PUT",
                _ => panic!("Not a valid method"),
            }
        }
    }

    let method = HttpMethod(Put);
    assert_eq!(method.as_str(), "PUT");
}

#[test]
fn test_as_str_options() {
    struct HttpMethod(Options);

    impl HttpMethod {
        pub fn as_str(&self) -> &str {
            match self.0 {
                Options => "OPTIONS",
                _ => panic!("Not a valid method"),
            }
        }
    }

    let method = HttpMethod(Options);
    assert_eq!(method.as_str(), "OPTIONS");
}

#[test]
fn test_as_str_get() {
    struct HttpMethod(Get);

    impl HttpMethod {
        pub fn as_str(&self) -> &str {
            match self.0 {
                Get => "GET",
                _ => panic!("Not a valid method"),
            }
        }
    }

    let method = HttpMethod(Get);
    assert_eq!(method.as_str(), "GET");
}

#[test]
fn test_as_str_post() {
    struct HttpMethod(Post);

    impl HttpMethod {
        pub fn as_str(&self) -> &str {
            match self.0 {
                Post => "POST",
                _ => panic!("Not a valid method"),
            }
        }
    }

    let method = HttpMethod(Post);
    assert_eq!(method.as_str(), "POST");
}

#[test]
fn test_as_str_delete() {
    struct HttpMethod(Delete);

    impl HttpMethod {
        pub fn as_str(&self) -> &str {
            match self.0 {
                Delete => "DELETE",
                _ => panic!("Not a valid method"),
            }
        }
    }

    let method = HttpMethod(Delete);
    assert_eq!(method.as_str(), "DELETE");
}

#[test]
fn test_as_str_head() {
    struct HttpMethod(Head);

    impl HttpMethod {
        pub fn as_str(&self) -> &str {
            match self.0 {
                Head => "HEAD",
                _ => panic!("Not a valid method"),
            }
        }
    }

    let method = HttpMethod(Head);
    assert_eq!(method.as_str(), "HEAD");
}

#[test]
fn test_as_str_trace() {
    struct HttpMethod(Trace);

    impl HttpMethod {
        pub fn as_str(&self) -> &str {
            match self.0 {
                Trace => "TRACE",
                _ => panic!("Not a valid method"),
            }
        }
    }

    let method = HttpMethod(Trace);
    assert_eq!(method.as_str(), "TRACE");
}

#[test]
fn test_as_str_connect() {
    struct HttpMethod(Connect);

    impl HttpMethod {
        pub fn as_str(&self) -> &str {
            match self.0 {
                Connect => "CONNECT",
                _ => panic!("Not a valid method"),
            }
        }
    }

    let method = HttpMethod(Connect);
    assert_eq!(method.as_str(), "CONNECT");
}

#[test]
fn test_as_str_patch() {
    struct HttpMethod(Patch);

    impl HttpMethod {
        pub fn as_str(&self) -> &str {
            match self.0 {
                Patch => "PATCH",
                _ => panic!("Not a valid method"),
            }
        }
    }

    let method = HttpMethod(Patch);
    assert_eq!(method.as_str(), "PATCH");
}

#[test]
fn test_as_str_extension_inline() {
    struct HttpMethod(ExtensionInline);

    impl HttpMethod {
        pub fn as_str(&self) -> &str {
            match self.0 {
                ExtensionInline(ref inline) => inline.as_str(),
                _ => panic!("Not a valid method"),
            }
        }
    }

    struct InlineString {
        string: String,
    }

    impl InlineString {
        pub fn as_str(&self) -> &str {
            &self.string
        }
    }

    let inline_method = InlineString { string: "INLINE_METHOD".to_string() };
    let method = HttpMethod(ExtensionInline(inline_method));
    assert_eq!(method.as_str(), "INLINE_METHOD");
}

#[test]
fn test_as_str_extension_allocated() {
    struct HttpMethod(ExtensionAllocated);

    impl HttpMethod {
        pub fn as_str(&self) -> &str {
            match self.0 {
                ExtensionAllocated(ref allocated) => allocated.as_str(),
                _ => panic!("Not a valid method"),
            }
        }
    }

    struct AllocatedString {
        string: String,
    }

    impl AllocatedString {
        pub fn as_str(&self) -> &str {
            &self.string
        }
    }

    let allocated_method = AllocatedString { string: "ALLOCATED_METHOD".to_string() };
    let method = HttpMethod(ExtensionAllocated(allocated_method));
    assert_eq!(method.as_str(), "ALLOCATED_METHOD");
}


// Answer 0

#[test]
fn test_authority_with_valid_authority() {
    let bytes = Bytes::from_static(b"example.com:80");
    let authority = Authority { data: ByteStr { bytes } };
    let uri = Uri { 
        scheme: Scheme { inner: Scheme2 }, 
        authority, 
        path_and_query: PathAndQuery { data: ByteStr { bytes: Bytes::from_static(b"/hello/world") }, query: 0 } 
    };
    
    assert_eq!(uri.authority().map(|a| a.data.bytes.as_ref()), Some(&b"example.com:80"[..]));
}

#[test]
fn test_authority_with_empty_authority() {
    let authority = Authority { data: ByteStr { bytes: Bytes::from_static(b"") } };
    let uri = Uri { 
        scheme: Scheme { inner: Scheme2 }, 
        authority, 
        path_and_query: PathAndQuery { data: ByteStr { bytes: Bytes::from_static(b"/hello/world") }, query: 0 } 
    };

    assert!(uri.authority().is_none());
}

#[test]
fn test_authority_with_only_host() {
    let bytes = Bytes::from_static(b"example.com");
    let authority = Authority { data: ByteStr { bytes } };
    let uri = Uri { 
        scheme: Scheme { inner: Scheme2 }, 
        authority, 
        path_and_query: PathAndQuery { data: ByteStr { bytes: Bytes::from_static(b"/hello/world") }, query: 0 } 
    };

    assert_eq!(uri.authority().map(|a| a.data.bytes.as_ref()), Some(&b"example.com"[..]));
}

#[test]
fn test_authority_with_username_password() {
    let bytes = Bytes::from_static(b"username:password@example.com:80");
    let authority = Authority { data: ByteStr { bytes } };
    let uri = Uri { 
        scheme: Scheme { inner: Scheme2 }, 
        authority, 
        path_and_query: PathAndQuery { data: ByteStr { bytes: Bytes::from_static(b"/hello/world") }, query: 0 } 
    };

    assert_eq!(uri.authority().map(|a| a.data.bytes.as_ref()), Some(&b"username:password@example.com:80"[..]));
}


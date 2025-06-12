// Answer 0

#[test]
fn test_path_and_query_slash() {
    use bytes::Bytes;
    
    #[derive(Clone)]
    pub struct PathAndQuery {
        pub(super) data: ByteStr,
        pub(super) query: u16,
    }
    
    #[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
    pub(crate) struct ByteStr {
        bytes: Bytes,
    }

    impl ByteStr {
        #[inline]
        pub const fn from_static(val: &'static str) -> ByteStr {
            ByteStr {
                bytes: Bytes::from_static(val.as_bytes()),
            }
        }
    }

    impl PathAndQuery {
        pub(super) fn slash() -> Self {
            PathAndQuery {
                data: ByteStr::from_static("/"),
                query: u16::MAX,
            }
        }
    }
    
    let result = PathAndQuery::slash();
    assert_eq!(result.data.bytes.as_ref(), b"/");
    assert_eq!(result.query, u16::MAX);
}


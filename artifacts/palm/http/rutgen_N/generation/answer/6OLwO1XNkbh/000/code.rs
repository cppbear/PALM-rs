// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use crate::uri::path::{PathAndQuery, NONE};
    use bytes::ByteStr;
    
    #[test]
    fn test_slash() {
        let path_and_query = slash();
        assert_eq!(path_and_query.data, ByteStr::from_static("/"));
        assert_eq!(path_and_query.query, NONE);
    }
}


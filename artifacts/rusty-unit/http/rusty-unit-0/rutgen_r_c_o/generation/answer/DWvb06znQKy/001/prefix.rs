// Answer 0

#[test]
fn test_query_none_with_empty_path() {
    let path_and_query = PathAndQuery::empty();
    path_and_query.query();
}

#[test]
fn test_query_none_with_slash() {
    let path_and_query = PathAndQuery::slash();
    path_and_query.query();
}

#[test]
fn test_query_none_with_star() {
    let path_and_query = PathAndQuery::star();
    path_and_query.query();
}

#[test]
fn test_query_none_with_static_string() {
    let path_and_query = PathAndQuery::from_static("/hello/world");
    path_and_query.query();
}

#[test]
fn test_query_none_with_maybe_shared_empty() {
    let src: &[u8] = b"/empty/path";
    let path_and_query = PathAndQuery::from_maybe_shared(src).unwrap();
    path_and_query.query();
}

#[test]
fn test_query_none_with_maybe_shared_no_query() {
    let src: &[u8] = b"/example/path";
    let path_and_query = PathAndQuery::from_maybe_shared(src).unwrap();
    path_and_query.query();
}


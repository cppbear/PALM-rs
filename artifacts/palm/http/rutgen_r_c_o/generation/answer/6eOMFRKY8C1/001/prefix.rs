// Answer 0

#[test]
fn test_path_and_query_valid_inputs() {
    let uri = uri::Builder::new()
        .path_and_query("/")
        .build()
        .unwrap();

    let uri = uri::Builder::new()
        .path_and_query("/?")
        .build()
        .unwrap();
    
    let uri = uri::Builder::new()
        .path_and_query("/hello")
        .build()
        .unwrap();
    
    let uri = uri::Builder::new()
        .path_and_query("/hello?foo=bar")
        .build()
        .unwrap();
    
    let uri = uri::Builder::new()
        .path_and_query("/hello?foo=bar&baz=qux")
        .build()
        .unwrap();
    
    let uri = uri::Builder::new()
        .path_and_query("/hello/world")
        .build()
        .unwrap();
    
    let uri = uri::Builder::new()
        .path_and_query("/path/with spaces")
        .build()
        .unwrap();

    let uri = uri::Builder::new()
        .path_and_query("/path/with/unicode/✓")
        .build()
        .unwrap();

    let uri = uri::Builder::new()
        .path_and_query("/very/long/path/that/should/be/handled/properly/and/might/cause/issues/if/not")
        .build()
        .unwrap();

    let uri = uri::Builder::new()
        .path_and_query("")
        .build()
        .unwrap();
    
    let uri = uri::Builder::new()
        .path_and_query("??")
        .build()
        .unwrap();

    let uri = uri::Builder::new()
        .path_and_query("hello%20world")
        .build()
        .unwrap();

    let uri = uri::Builder::new()
        .path_and_query("path#fragment")
        .build()
        .unwrap();

    let uri = uri::Builder::new()
        .path_and_query("path;param")
        .build()
        .unwrap();
    
    let uri = uri::Builder::new()
        .path_and_query("/path/with/invalid/character/<>@")
        .build()
        .unwrap();

    let uri = uri::Builder::new()
        .path_and_query("//double/slash")
        .build()
        .unwrap();

    let uri = uri::Builder::new()
        .path_and_query("/path/with/multiple//slashes")
        .build()
        .unwrap();
    
    let uri = uri::Builder::new()
        .path_and_query(" ")
        .build()
        .unwrap();
}


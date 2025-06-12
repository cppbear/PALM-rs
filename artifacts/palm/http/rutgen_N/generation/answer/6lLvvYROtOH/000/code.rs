// Answer 0

#[test]
fn test_from_static_valid_header() {
    struct TestHdrName<'a>(&'a str);
    
    let result = from_static("Content-Type", |hdr: HdrName| {
        assert_eq!(hdr, TestHdrName("Content-Type"));
    });
}

#[test]
fn test_from_static_invalid_header() {
    #[should_panic(expected = "static str is invalid name")]
    fn test_invalid() {
        from_static("Invalid-Header!", |hdr: HdrName| {});
    }
    
    test_invalid();
}


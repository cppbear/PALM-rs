// Answer 0

#[test]
fn test_into_parts_with_empty_response() {
    let response: Response<()> = Response::new(());
    let (parts, body) = response.into_parts();
}

#[test]
fn test_into_parts_with_integer_body() {
    let response: Response<i32> = Response::new(42);
    let (parts, body) = response.into_parts();
}

#[test]
fn test_into_parts_with_string_body() {
    let response: Response<String> = Response::new(String::from("Hello, World!"));
    let (parts, body) = response.into_parts();
}

#[test]
fn test_into_parts_with_custom_struct_body() {
    #[derive(Debug)]
    struct CustomStruct {
        data: i32,
    }
    let response: Response<CustomStruct> = Response::new(CustomStruct { data: 10 });
    let (parts, body) = response.into_parts();
}

#[test]
fn test_into_parts_with_response_from_parts() {
    let parts = Parts {
        status: StatusCode::OK,
        version: Version::HTTP_11,
        headers: HeaderMap::new(),
        extensions: Extensions::new(),
        _priv: (),
    };
    let response: Response<()> = Response::from_parts(parts.clone(), ());
    let (returned_parts, body) = response.into_parts();
}

#[test]
fn test_into_parts_with_different_response_body() {
    let parts = Parts {
        status: StatusCode::NOT_FOUND,
        version: Version::HTTP_11,
        headers: HeaderMap::new(),
        extensions: Extensions::new(),
        _priv: (),
    };
    let response: Response<()> = Response::from_parts(parts, ());
    let (parts, body) = response.into_parts();
}


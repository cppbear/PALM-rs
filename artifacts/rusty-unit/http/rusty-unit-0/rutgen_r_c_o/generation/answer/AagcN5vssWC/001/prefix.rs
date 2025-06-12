// Answer 0

#[test]
fn test_status_default() {
    let response: Response<()> = Response::new(());
    let status_code = response.status();
}

#[test]
fn test_status_custom_code() {
    let parts = Parts {
        status: StatusCode(200.try_into().unwrap()),
        version: Version::default(),
        headers: HeaderMap::new(),
        extensions: Extensions::default(),
        _priv: (),
    };
    let response = Response::from_parts(parts, ());
    let status_code = response.status();
}

#[test]
fn test_status_minimum_value() {
    let parts = Parts {
        status: StatusCode(1.try_into().unwrap()),
        version: Version::default(),
        headers: HeaderMap::new(),
        extensions: Extensions::default(),
        _priv: (),
    };
    let response = Response::from_parts(parts, ());
    let status_code = response.status();
}

#[test]
fn test_status_maximum_value() {
    let parts = Parts {
        status: StatusCode(65535.try_into().unwrap()),
        version: Version::default(),
        headers: HeaderMap::new(),
        extensions: Extensions::default(),
        _priv: (),
    };
    let response = Response::from_parts(parts, ());
    let status_code = response.status();
}


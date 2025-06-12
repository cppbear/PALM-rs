// Answer 0

#[test]
fn test_version_http_0_9() {
    let parts = Parts {
        version: Version(Http::HTTP_0_9),
        ..Default::default()
    };
    let request: Request<()> = Request::from_parts(parts, ());
    request.version();
}

#[test]
fn test_version_http_1_0() {
    let parts = Parts {
        version: Version(Http::HTTP_1_0),
        ..Default::default()
    };
    let request: Request<()> = Request::from_parts(parts, ());
    request.version();
}

#[test]
fn test_version_http_1_1() {
    let parts = Parts {
        version: Version(Http::HTTP_1_1),
        ..Default::default()
    };
    let request: Request<()> = Request::from_parts(parts, ());
    request.version();
}

#[test]
fn test_version_http_2_0() {
    let parts = Parts {
        version: Version(Http::HTTP_2_0),
        ..Default::default()
    };
    let request: Request<()> = Request::from_parts(parts, ());
    request.version();
}

#[test]
fn test_version_http_3() {
    let parts = Parts {
        version: Version(Http::HTTP_3),
        ..Default::default()
    };
    let request: Request<()> = Request::from_parts(parts, ());
    request.version();
}


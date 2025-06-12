// Answer 0

#[test]
fn test_response_from_parts() {
    struct DummyBody;
    
    let parts = Parts {
        status: StatusCode::OK,
        version: Version::HTTP_11,
        headers: HeaderMap::new(),
        extensions: Extensions::new(),
        _priv: (),
    };

    let body = DummyBody;
    
    let response = Response::from_parts(parts.clone(), body);
    
    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.version(), Version::HTTP_11);
    assert_eq!(response.headers().len(), 0);
}

#[test]
fn test_response_body_mutability() {
    struct DummyBody {
        value: String,
    }

    let parts = Parts {
        status: StatusCode::OK,
        version: Version::HTTP_11,
        headers: HeaderMap::new(),
        extensions: Extensions::new(),
        _priv: (),
    };
    
    let body = DummyBody {
        value: "initial".to_string(),
    };

    let mut response = Response::from_parts(parts, body);
    
    {
        let body_mut = response.body_mut();
        body_mut.value = "updated".to_string();
    }

    assert_eq!(response.body().value, "updated");
}

#[test]
fn test_response_into_parts() {
    struct DummyBody {
        data: i32,
    }
    
    let parts = Parts {
        status: StatusCode::ACCEPTED,
        version: Version::HTTP_11,
        headers: HeaderMap::new(),
        extensions: Extensions::new(),
        _priv: (),
    };

    let body = DummyBody { data: 42 };

    let response = Response::from_parts(parts, body);
    
    let (retrieved_parts, retrieved_body) = response.into_parts();

    assert_eq!(retrieved_parts.status, StatusCode::ACCEPTED);
    assert_eq!(retrieved_body.data, 42);
}

#[test]
fn test_response_map_functionality() {
    struct DummyBody {
        value: i32,
    }

    let parts = Parts {
        status: StatusCode::NO_CONTENT,
        version: Version::HTTP_11,
        headers: HeaderMap::new(),
        extensions: Extensions::new(),
        _priv: (),
    };
    
    let body = DummyBody { value: 100 };

    let response = Response::from_parts(parts, body);
    
    let new_response: Response<String> = response.map(|b| b.value.to_string());

    assert_eq!(new_response.status(), StatusCode::NO_CONTENT);
    assert_eq!(*new_response.body(), "100");
}


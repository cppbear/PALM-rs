// Answer 0

#[test]
fn test_from_parts_creates_request() {
    struct Parts {
        method: Method,
    }

    struct Request<T> {
        head: Parts,
        body: T,
    }

    struct Method {
        name: &'static str,
    }

    impl Method {
        const POST: Method = Method { name: "POST" };
    }

    let parts = Parts {
        method: Method::POST,
    };

    let body = "request body";

    let request = from_parts(parts, body);

    assert_eq!(request.head.method.name, "POST");
    assert_eq!(request.body, "request body");
}

#[test]
#[should_panic]
fn test_from_parts_panic_on_invalid_state() {
    struct Parts {
        method: Method,
    }

    struct Request<T> {
        head: Parts,
        body: Result<T, &'static str>,
    }

    struct Method {
        name: &'static str,
    }

    impl Method {
        const GET: Method = Method { name: "GET" };
    }

    let parts = Parts {
        method: Method::GET,
    };

    let body: Result<String, &'static str> = Err("Invalid body");

    let _request = from_parts(parts, body);
}


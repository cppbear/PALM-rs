// Answer 0

#[derive(Default)]
struct Response<T> {
    head: Parts,
    body: T,
}

#[derive(Default)]
struct Parts {
    status: StatusCode,
}

#[derive(Default, PartialEq)]
struct StatusCode {
    code: u16,
}

impl StatusCode {
    const OK: StatusCode = StatusCode { code: 200 };
}

impl<T: Default> Response<T> {
    pub fn into_parts(self) -> (Parts, T) {
        (self.head, self.body)
    }
}

#[test]
fn test_into_parts_default_response() {
    let response: Response<()> = Response::default();
    let (parts, body) = response.into_parts();
    assert_eq!(parts.status, StatusCode::OK);
}

#[test]
fn test_into_parts_with_non_default_body() {
    struct CustomBody {
        data: String,
    }

    let response = Response {
        head: Parts {
            status: StatusCode::OK,
        },
        body: CustomBody {
            data: String::from("example"),
        },
    };

    let (parts, body) = response.into_parts();
    assert_eq!(parts.status, StatusCode::OK);
    assert_eq!(body.data, "example");
}

#[test]
fn test_into_parts_empty_body() {
    let response: Response<()> = Response {
        head: Parts {
            status: StatusCode::OK,
        },
        body: (),
    };

    let (parts, body) = response.into_parts();
    assert_eq!(parts.status, StatusCode::OK);
}


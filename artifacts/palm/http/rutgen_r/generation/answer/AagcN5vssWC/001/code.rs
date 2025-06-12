// Answer 0

#[derive(Default)]
struct Response<T> {
    head: Head<T>,
}

#[derive(Default)]
struct Head<T> {
    status: StatusCode,
}

#[derive(Debug, PartialEq)]
enum StatusCode {
    OK,
    NotFound,
    InternalServerError,
}

impl<T> Response<T> {
    pub fn status(&self) -> StatusCode {
        self.head.status
    }
}

#[test]
fn test_response_status_ok() {
    let response: Response<()> = Response::<()>::default();
    assert_eq!(response.status(), StatusCode::OK);
}

#[test]
fn test_response_status_not_found() {
    let mut response: Response<()> = Response::<()>::default();
    response.head.status = StatusCode::NotFound;
    assert_eq!(response.status(), StatusCode::NotFound);
}

#[test]
fn test_response_status_internal_server_error() {
    let mut response: Response<()> = Response::<()>::default();
    response.head.status = StatusCode::InternalServerError;
    assert_eq!(response.status(), StatusCode::InternalServerError);
}

#[test]
#[should_panic]
fn test_response_status_panics_when_uninitialized() {
    let response: Response<()> = Response::default();
    assert_eq!(response.status(), StatusCode::InternalServerError); // This should panic if status is not OK
}


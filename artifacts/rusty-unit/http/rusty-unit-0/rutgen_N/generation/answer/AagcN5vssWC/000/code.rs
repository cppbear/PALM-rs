// Answer 0

#[test]
fn test_response_status_default() {
    struct Response<T> {
        head: Head<T>,
    }

    struct Head<T> {
        status: StatusCode,
    }

    impl<T> Default for Response<T> {
        fn default() -> Self {
            Response {
                head: Head {
                    status: StatusCode::OK,
                },
            }
        }
    }

    #[derive(PartialEq, Debug)]
    enum StatusCode {
        OK,
    }

    let response: Response<()> = Response::default();
    assert_eq!(response.status(), StatusCode::OK);
}


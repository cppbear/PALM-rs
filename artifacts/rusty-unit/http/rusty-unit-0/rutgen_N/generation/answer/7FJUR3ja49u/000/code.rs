// Answer 0

#[test]
fn test_status_mut_initialization() {
    struct Response<T> {
        head: Head<T>,
    }

    struct Head<T> {
        status: StatusCode,
        _marker: std::marker::PhantomData<T>,
    }

    #[derive(Debug, PartialEq)]
    enum StatusCode {
        OK,
        CREATED,
    }

    impl<T> Default for Response<T> {
        fn default() -> Self {
            Response {
                head: Head {
                    status: StatusCode::OK,
                    _marker: std::marker::PhantomData,
                },
            }
        }
    }

    impl<T> Response<T> {
        pub fn status_mut(&mut self) -> &mut StatusCode {
            &mut self.head.status
        }

        pub fn status(&self) -> &StatusCode {
            &self.head.status
        }
    }

    let mut response: Response<()> = Response::default();
    *response.status_mut() = StatusCode::CREATED;
    assert_eq!(response.status(), &StatusCode::CREATED);
}

#[test]
fn test_status_mut_boundary_value() {
    struct Response<T> {
        head: Head<T>,
    }

    struct Head<T> {
        status: StatusCode,
        _marker: std::marker::PhantomData<T>,
    }

    #[derive(Debug, PartialEq)]
    enum StatusCode {
        OK,
        CREATED,
    }

    impl<T> Default for Response<T> {
        fn default() -> Self {
            Response {
                head: Head {
                    status: StatusCode::OK,
                    _marker: std::marker::PhantomData,
                },
            }
        }
    }

    impl<T> Response<T> {
        pub fn status_mut(&mut self) -> &mut StatusCode {
            &mut self.head.status
        }

        pub fn status(&self) -> &StatusCode {
            &self.head.status
        }
    }

    let mut response: Response<()> = Response::default();
    *response.status_mut() = StatusCode::CREATED;
    assert_eq!(response.status(), &StatusCode::CREATED);
    
    *response.status_mut() = StatusCode::OK;
    assert_eq!(response.status(), &StatusCode::OK);
}


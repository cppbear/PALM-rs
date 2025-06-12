// Answer 0

#[derive(Debug)]
struct Builder {
    status: Option<StatusCode>,
}

#[derive(Debug)]
struct StatusCode(u16);

impl TryInto<StatusCode> for u16 {
    type Error = ();

    fn try_into(self) -> Result<StatusCode, Self::Error> {
        if self >= 100 && self <= 599 {
            Ok(StatusCode(self))
        } else {
            Err(())
        }
    }
}

impl Builder {
    fn new() -> Self {
        Self { status: None }
    }

    fn status<T>(mut self, status: T) -> Result<Self, ()>
    where
        T: TryInto<StatusCode, Error = ()>,
    {
        let status_code = status.try_into()?;
        self.status = Some(status_code);
        Ok(self)
    }
}

#[test]
fn test_status_success() {
    let builder = Builder::new();
    let result = builder.status(200);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().status, Some(StatusCode(200)));
}

#[test]
fn test_status_out_of_range_low() {
    let builder = Builder::new();
    let result = builder.status(99);
    assert!(result.is_err());
}

#[test]
fn test_status_out_of_range_high() {
    let builder = Builder::new();
    let result = builder.status(600);
    assert!(result.is_err());
}

#[test]
fn test_status_non_integer() {
    let builder = Builder::new();
    // Assuming we're treating non-integers which can technically happen via some conversions.
    let result = builder.status("not a number");
    assert!(result.is_err());
}


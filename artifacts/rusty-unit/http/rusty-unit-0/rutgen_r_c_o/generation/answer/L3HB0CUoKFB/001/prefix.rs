// Answer 0

#[test]
fn test_into_body_with_zero() {
    let response = Response::new(0);
    response.into_body();
}

#[test]
fn test_into_body_with_one() {
    let response = Response::new(1);
    response.into_body();
}

#[test]
fn test_into_body_with_five() {
    let response = Response::new(5);
    response.into_body();
}

#[test]
fn test_into_body_with_ten() {
    let response = Response::new(10);
    response.into_body();
}

#[test]
fn test_into_body_with_negative_value() {
    let response = Response::new(-1);
    response.into_body();
}

#[test]
fn test_into_body_with_large_value() {
    let response = Response::new(100);
    response.into_body();
}


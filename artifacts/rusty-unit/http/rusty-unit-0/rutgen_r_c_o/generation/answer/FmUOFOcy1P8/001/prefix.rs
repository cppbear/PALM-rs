// Answer 0

#[test]
fn test_fmt_with_success_status() {
    let status = StatusCode::OK;
    let mut output = String::new();
    write!(&mut output, "{}", status).unwrap();
}

#[test]
fn test_fmt_with_informational_status() {
    let status = StatusCode::CONTINUE;
    let mut output = String::new();
    write!(&mut output, "{}", status).unwrap();
}

#[test]
fn test_fmt_with_redirection_status() {
    let status = StatusCode::MOVED_PERMANENTLY;
    let mut output = String::new();
    write!(&mut output, "{}", status).unwrap();
}

#[test]
fn test_fmt_with_client_error_status() {
    let status = StatusCode::BAD_REQUEST;
    let mut output = String::new();
    write!(&mut output, "{}", status).unwrap();
}

#[test]
fn test_fmt_with_server_error_status() {
    let status = StatusCode::INTERNAL_SERVER_ERROR;
    let mut output = String::new();
    write!(&mut output, "{}", status).unwrap();
}

#[test]
fn test_fmt_with_boundary_case_below() {
    let status = StatusCode::from_u16(100).unwrap();
    let mut output = String::new();
    write!(&mut output, "{}", status).unwrap();
}

#[test]
fn test_fmt_with_boundary_case_at() {
    let status = StatusCode::from_u16(511).unwrap();
    let mut output = String::new();
    write!(&mut output, "{}", status).unwrap();
}

#[test]
fn test_fmt_with_undefined_status() {
    let status = StatusCode::from_u16(999);
    let mut output = String::new();
    if let Err(_) = status {
        write!(&mut output, "<unknown status code>").unwrap();
    }
}


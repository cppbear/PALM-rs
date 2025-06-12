// Answer 0

#[test]
fn test_host_with_ipv6() {
    let auth = "[::1]:8080@user";
    let result = host(auth);
    assert_eq!(result, "[::1]");
}

#[test]
fn test_host_without_port() {
    let auth = "user@localhost";
    let result = host(auth);
    assert_eq!(result, "localhost");
}

#[test]
fn test_host_with_port() {
    let auth = "user@localhost:8080";
    let result = host(auth);
    assert_eq!(result, "localhost");
}

#[test]
fn test_host_with_empty_input() {
    let auth = "@localhost";
    let result = host(auth);
    assert_eq!(result, "localhost");
}

#[should_panic(expected = "split always has at least 1 item")]
#[test]
fn test_host_with_no_auth_part() {
    let auth = "";
    host(auth);
}

#[should_panic(expected = "parsing should validate brackets")]
#[test]
fn test_host_with_invalid_ipv6() {
    let auth = "[::1:8080]:user";
    host(auth);
}

#[test]
fn test_host_with_multiple_colons() {
    let auth = "user@[fe80::1%eth0]:80@router";
    let result = host(auth);
    assert_eq!(result, "[fe80::1%eth0]");
}


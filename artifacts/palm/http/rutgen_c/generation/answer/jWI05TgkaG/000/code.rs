// Answer 0

#[test]
fn test_host_with_ip_address() {
    let auth = "[192.168.0.1]:8080";
    let result = host(auth);
    assert_eq!(result, "[192.168.0.1]");
}

#[test]
fn test_host_with_hostname() {
    let auth = "example.com:80";
    let result = host(auth);
    assert_eq!(result, "example.com");
}

#[test]
fn test_host_with_authentication_info() {
    let auth = "user:password@example.com:80";
    let result = host(auth);
    assert_eq!(result, "example.com");
}

#[test]
fn test_host_with_brackets() {
    let auth = "[::1]:80";
    let result = host(auth);
    assert_eq!(result, "[::1]");
}

#[test]
#[should_panic(expected = "split always has at least 1 item")]
fn test_host_with_empty_string() {
    let auth = "";
    let _ = host(auth);
}

#[test]
#[should_panic(expected = "parsing should validate brackets")]
fn test_host_with_invalid_brackets() {
    let auth = "[192.168.0.1:80"; // missing closing bracket
    let _ = host(auth);
}


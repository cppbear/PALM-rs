// Answer 0

#[test]
fn test_host_with_ipv6_address() {
    let auth = "[::1]:8080";
    let result = host(auth);
    assert_eq!(result, "[::1]");
}

#[test]
fn test_host_with_username_and_ipv6_address() {
    let auth = "user@[::1]:8080";
    let result = host(auth);
    assert_eq!(result, "[::1]");
}

#[test]
fn test_host_with_simple_host() {
    let auth = "example.com:80";
    let result = host(auth);
    assert_eq!(result, "example.com");
}

#[test]
fn test_host_with_username_and_simple_host() {
    let auth = "user@example.com:80";
    let result = host(auth);
    assert_eq!(result, "example.com");
}

#[test]
#[should_panic(expected = "split always has at least 1 item")]
fn test_host_with_no_auth_case() {
    let auth = "";
    host(auth);
}

#[test]
#[should_panic(expected = "split always has at least 1 item")]
fn test_host_with_only_at_symbol() {
    let auth = "@";
    host(auth);
}

#[test]
#[should_panic(expected = "parsing should validate brackets")]
fn test_host_with_unmatched_brackets() {
    let auth = "[::1:8080";
    host(auth);
}


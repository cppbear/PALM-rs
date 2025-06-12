// Answer 0

#[test]
fn test_host_with_valid_host() {
    let auth = "user@example.com:80";
    let result = host(auth);
    assert_eq!(result, "user@example.com");
}

#[test]
fn test_host_with_ipv6_address() {
    let auth = "[2001:db8::1]:80";
    let result = host(auth);
    assert_eq!(result, "[2001:db8::1]");
}

#[test]
fn test_host_with_ipv6_address_no_port() {
    let auth = "[2001:db8::1]";
    let result = host(auth);
    assert_eq!(result, "[2001:db8::1]");
}

#[test]
#[should_panic(expected = "split always has at least 1 item")]
fn test_host_with_no_auth() {
    let auth = "";
    host(auth);
}

#[test]
#[should_panic(expected = "parsing should validate brackets")]
fn test_host_with_incomplete_ipv6_address() {
    let auth = "[2001:db8::1:80"; // Missing closing bracket
    host(auth);
}

#[test]
#[should_panic(expected = "split always has at least 1 item")]
fn test_host_with_empty_userinfo() {
    let auth = "@example.com:80";
    host(auth);
}


// Answer 0

#[test]
fn test_host_with_valid_ipv4() {
    let auth = "user:password@192.168.1.1:8080";
    let result = host(auth);
    assert_eq!(result, "192.168.1.1");
}

#[test]
fn test_host_with_valid_ipv6() {
    let auth = "user:password@[2001:db8::ff00:42:8329]:8080";
    let result = host(auth);
    assert_eq!(result, "[2001:db8::ff00:42:8329]");
}

#[test]
fn test_host_with_no_port() {
    let auth = "user:password@hostname";
    let result = host(auth);
    assert_eq!(result, "hostname");
}

#[test]
fn test_host_with_empty() {
    let auth = "user:password@";
    let result = host(auth);
    assert_eq!(result, "");
}

#[test]
#[should_panic(expected = "split always has at least 1 item")]
fn test_host_without_at_symbol() {
    let auth = "user:password";
    host(auth);
}

#[test]
#[should_panic(expected = "parsing should validate brackets")]
fn test_host_with_invalid_ipv6() {
    let auth = "user:password@[2001:db8::ff00:42:8329:8080";
    host(auth);
}


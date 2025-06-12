// Answer 0

#[test]
fn test_host_with_ipv6() {
    let auth = "[2001:db8::1]:8080";
    let result = host(auth);
    assert_eq!(result, "[2001:db8::1]");
}

#[test]
fn test_host_with_ipv4() {
    let auth = "user:password@192.168.1.1:8080";
    let result = host(auth);
    assert_eq!(result, "192.168.1.1");
}

#[test]
fn test_host_without_port() {
    let auth = "user:password@localhost";
    let result = host(auth);
    assert_eq!(result, "localhost");
}

#[test]
fn test_host_with_empty_string() {
    let auth = "";
    let result = host(auth);
    assert_eq!(result, "");
}

#[test]
#[should_panic(expected = "split always has at least 1 item")]
fn test_host_with_only_user_info() {
    let auth = "user:password@";
    let _ = host(auth);
}


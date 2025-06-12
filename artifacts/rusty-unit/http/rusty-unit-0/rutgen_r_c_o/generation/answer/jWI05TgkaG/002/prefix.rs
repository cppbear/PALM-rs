// Answer 0

#[test]
fn test_host_with_ipv4_address_no_port() {
    let auth = "user@example.com@192.168.1.1";
    let result = host(auth);
}

#[test]
fn test_host_with_ipv6_address_with_brackets_no_port() {
    let auth = "user@example.com@[::1]";
    let result = host(auth);
}

#[test]
fn test_host_with_ipv6_address_with_brackets_with_port() {
    let auth = "user@example.com@[::1]:8080";
    let result = host(auth);
}

#[test]
fn test_host_with_hostname_no_port() {
    let auth = "user@example.com@hostname.com";
    let result = host(auth);
}

#[test]
fn test_host_with_hostname_with_port() {
    let auth = "user@example.com@hostname.com:80";
    let result = host(auth);
}

#[test]
fn test_host_with_ipv4_address_with_port() {
    let auth = "user@example.com@10.0.0.1:3000";
    let result = host(auth);
}

#[test]
fn test_host_with_ipv6_address_only() {
    let auth = "@[2001:db8::1]";
    let result = host(auth);
}

#[test]
fn test_host_split_first_segment() {
    let auth = "user@host:1234";
    let result = host(auth);
}

#[test]
fn test_host_empty_userinfo() {
    let auth = "@hostname.com";
    let result = host(auth);
}


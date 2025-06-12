// Answer 0

#[test]
fn test_host_user_host_port() {
    let auth = "user@host:port";
    let result = host(auth);
}

#[test]
fn test_host_user_square_brackets() {
    let auth = "user@[host]:port";
    let result = host(auth);
}

#[test]
fn test_host_user_host() {
    let auth = "user@host";
    let result = host(auth);
}

#[test]
#[should_panic(expected = "parsing should validate brackets")]
fn test_host_user_missing_closing_bracket() {
    let auth = "user@[host";
    let result = host(auth);
}

#[test]
fn test_host_user_square_brackets_with_port() {
    let auth = "user@[host]:port";
    let result = host(auth);
}

#[test]
fn test_host_plain_host_port() {
    let auth = "host:port";
    let result = host(auth);
}

#[test]
fn test_host_plain_host() {
    let auth = "host";
    let result = host(auth);
}


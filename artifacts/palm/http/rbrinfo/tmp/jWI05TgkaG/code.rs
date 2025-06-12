fn host(auth: &str) -> &str {
    let host_port = auth
        .rsplit('@')
        .next()
        .expect("split always has at least 1 item");

    if host_port.as_bytes()[0] == b'[' {
        let i = host_port
            .find(']')
            .expect("parsing should validate brackets");
        // ..= ranges aren't available in 1.20, our minimum Rust version...
        &host_port[0..i + 1]
    } else {
        host_port
            .split(':')
            .next()
            .expect("split always has at least 1 item")
    }
}
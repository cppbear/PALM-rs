// Answer 0

#[derive(Debug)]
struct Http(u8);

impl Http {
    const Http09: u8 = 0;
    const Http10: u8 = 1;
    const Http11: u8 = 2;
    const H2: u8 = 3;
    const H3: u8 = 4;
    const __NonExhaustive: u8 = 255;

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use self::Http::*;

        f.write_str(match self.0 {
            Http09 => "HTTP/0.9",
            Http10 => "HTTP/1.0",
            Http11 => "HTTP/1.1",
            H2 => "HTTP/2.0",
            H3 => "HTTP/3.0",
            __NonExhaustive => unreachable!(),
        })
    }
}

#[test]
fn test_http_10_fmt() {
    let http10 = Http(Http::Http10);
    let mut output = String::new();
    let result = http10.fmt(&mut std::fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(output, "HTTP/1.0");
}

#[test]
fn test_http_09_fmt() {
    let http09 = Http(Http::Http09);
    let mut output = String::new();
    let result = http09.fmt(&mut std::fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(output, "HTTP/0.9");
}

#[test]
fn test_http_11_fmt() {
    let http11 = Http(Http::Http11);
    let mut output = String::new();
    let result = http11.fmt(&mut std::fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(output, "HTTP/1.1");
}

#[test]
fn test_http_2_fmt() {
    let http2 = Http(Http::H2);
    let mut output = String::new();
    let result = http2.fmt(&mut std::fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(output, "HTTP/2.0");
}

#[test]
fn test_http_3_fmt() {
    let http3 = Http(Http::H3);
    let mut output = String::new();
    let result = http3.fmt(&mut std::fmt::Formatter::new(&mut output));
    assert!(result.is_ok());
    assert_eq!(output, "HTTP/3.0");
}


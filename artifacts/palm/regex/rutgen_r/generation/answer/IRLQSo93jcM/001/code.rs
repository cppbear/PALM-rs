// Answer 0

#[test]
fn test_bytes_true() {
    struct Config {
        bytes: bool,
    }

    let config = Config { bytes: false };
    let result = config.bytes(true);
    assert_eq!(result.bytes, true);
}

#[test]
fn test_bytes_false() {
    struct Config {
        bytes: bool,
    }

    let config = Config { bytes: true };
    let result = config.bytes(false);
    assert_eq!(result.bytes, false);
}

#[test]
fn test_bytes_multiple_calls() {
    struct Config {
        bytes: bool,
    }

    let config = Config { bytes: false };
    let result = config.bytes(true).bytes(false);
    assert_eq!(result.bytes, false);
}


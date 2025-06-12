// Answer 0

#[test]
fn test_fmt_range() {
    let entries = [
        Bucket { hash: 1, key: 0, value: "a" },
        Bucket { hash: 1, key: 1, value: "b" },
        Bucket { hash: 1, key: 2, value: "c" },
        Bucket { hash: 1, key: 3, value: "d" },
        Bucket { hash: 1, key: 4, value: "e" },
        Bucket { hash: 1, key: 5, value: "f" },
        Bucket { hash: 1, key: 6, value: "g" },
        Bucket { hash: 1, key: 7, value: "h" },
        Bucket { hash: 1, key: 8, value: "i" },
        Bucket { hash: 1, key: 9, value: "j" },
    ];
    let slice = Slice { entries };
    let _ = slice.fmt(&mut fmt::Formatter::new());
}

#[test]
fn test_fmt_range_from() {
    let entries = [
        Bucket { hash: 1, key: 50, value: "a" },
        Bucket { hash: 1, key: 51, value: "b" },
        Bucket { hash: 1, key: 52, value: "c" },
        Bucket { hash: 1, key: 53, value: "d" },
    ];
    let slice = Slice { entries };
    let _ = slice.fmt(&mut fmt::Formatter::new());
}

#[test]
fn test_fmt_range_inclusive() {
    let entries = [
        Bucket { hash: 1, key: 0, value: "a" },
        Bucket { hash: 1, key: 1, value: "b" },
        Bucket { hash: 1, key: 2, value: "c" },
        Bucket { hash: 1, key: 3, value: "d" },
        Bucket { hash: 1, key: 4, value: "e" },
        Bucket { hash: 1, key: 5, value: "f" },
    ];
    let slice = Slice { entries };
    let _ = slice.fmt(&mut fmt::Formatter::new());
}

#[test]
fn test_fmt_range_to() {
    let entries = [
        Bucket { hash: 1, key: 0, value: "a" },
        Bucket { hash: 1, key: 1, value: "b" },
        Bucket { hash: 1, key: 2, value: "c" },
        Bucket { hash: 1, key: 3, value: "d" },
        Bucket { hash: 1, key: 30, value: "e" },
    ];
    let slice = Slice { entries };
    let _ = slice.fmt(&mut fmt::Formatter::new());
}

#[test]
fn test_fmt_range_to_inclusive() {
    let entries = [
        Bucket { hash: 1, key: 10, value: "a" },
        Bucket { hash: 1, key: 20, value: "b" },
        Bucket { hash: 1, key: 30, value: "c" },
        Bucket { hash: 1, key: 40, value: "d" },
        Bucket { hash: 1, key: 50, value: "e" },
    ];
    let slice = Slice { entries };
    let _ = slice.fmt(&mut fmt::Formatter::new());
}

#[test]
fn test_fmt_bound() {
    let entries = [
        Bucket { hash: 1, key: 5, value: "a" },
        Bucket { hash: 1, key: 10, value: "b" },
        Bucket { hash: 1, key: 12, value: "c" },
        Bucket { hash: 1, key: 15, value: "d" },
    ];
    let slice = Slice { entries };
    let _ = slice.fmt(&mut fmt::Formatter::new());
}


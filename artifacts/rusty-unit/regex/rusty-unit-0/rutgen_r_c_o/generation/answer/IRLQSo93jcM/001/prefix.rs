// Answer 0

#[test]
fn test_bytes_true() {
    let builder = ExecBuilder::new("test");
    let _ = builder.bytes(true);
}

#[test]
fn test_bytes_false() {
    let builder = ExecBuilder::new("example");
    let _ = builder.bytes(false);
}

#[test]
fn test_bytes_true_again() {
    let builder = ExecBuilder::new_many(vec!["sample1", "sample2"]);
    let _ = builder.bytes(true);
}

#[test]
fn test_bytes_multiple_calls() {
    let builder = ExecBuilder::new("test");
    let _ = builder.bytes(true);
    let _ = builder.bytes(false);
}

#[test]
fn test_bytes_edge_case_false() {
    let mut builder = ExecBuilder::new("edge_case");
    builder = builder.bytes(false);
    let _ = builder.bytes(false);
}

#[test]
fn test_bytes_edge_case_true() {
    let mut builder = ExecBuilder::new("edge_case");
    builder = builder.bytes(true);
    let _ = builder.bytes(true);
}


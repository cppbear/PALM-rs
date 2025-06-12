// Answer 0

#[test]
fn test_new_with_valid_engine() {
    struct MockEngine;
    
    let engine = MockEngine;
    let encoder = base64::new(&engine);
    
    // Since we expect a ChunkedEncoder to be returned, we can assert it does not panic
    // In this case, we don't have specific properties to test on ChunkedEncoder as no methods 
    // are provided in the context, but we just check that it's safely created.
    assert!(std::mem::size_of_val(&encoder) > 0);
}

#[test]
#[should_panic(expected = "attempt to create ChunkedEncoder with null engine")] // simulate a panic condition
fn test_new_with_null_engine() {
    // Here we would typically pass None or a similar concept if the function allowed nulls,
    // but since Rust's borrowing doesn't allow it directly, we use another approach without a formal type.
    let encoder: base64::ChunkedEncoder<_>;
    encoder = base64::new(std::ptr::null()); // this is just a simulated panic; 
    // In real use, the method signature could prevent this from happening or not compile at all
}


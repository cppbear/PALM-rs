// Answer 0

#[test]
fn test_status_mut() {
    use std::num::NonZeroU16;
    use crate::{Response, Parts, StatusCode, HeaderMap, Version, Extensions, Method, Uri};

    // Create a dummy header map, version, and extensions
    let headers = HeaderMap::new();
    let version = Version::HTTP_11; // Assume HTTP_11 is a valid variant
    let extensions = Extensions::new();
    let method = Method::GET; // Assume Method::GET is a valid variant
    let uri = Uri::default(); // Assume default() is a valid method for Uri
    
    // Create Parts with a status code of 200 (OK)
    let status = StatusCode(NonZeroU16::new(200).unwrap());
    let parts = Parts {
        status,
        version,
        headers,
        extensions,
        _priv: (),
    };

    // Initialize a Response with body type ()
    let mut response: Response<()> = Response::from_parts(parts, ());

    // Check that initially status is 200 (OK)
    assert_eq!(response.status().0.get(), 200);

    // Mutate the status to 201 (Created)
    *response.status_mut() = StatusCode(NonZeroU16::new(201).unwrap());

    // Verify that the status has changed to 201 (Created)
    assert_eq!(response.status().0.get(), 201);
}

#[test]
#[should_panic]
fn test_status_mut_invalid_status_code() {
    use std::num::NonZeroU16;
    use crate::{Response, Parts, StatusCode, HeaderMap, Version, Extensions, Method, Uri};

    let headers = HeaderMap::new();
    let version = Version::HTTP_11; // Assume valid version
    let extensions = Extensions::new();
    let method = Method::GET; // Assume valid method
    let uri = Uri::default(); // Assume valid Uri
    
    // Create Parts with invalid status code which should panic
    let status = StatusCode(NonZeroU16::new(0).unwrap()); // 0 is not a valid status code
    let parts = Parts {
        status,
        version,
        headers,
        extensions,
        _priv: (),
    };

    // Initialize a Response with body type ()
    let mut response: Response<()> = Response::from_parts(parts, ());

    // This operation is expected to panic
    let _ = response.status_mut(); // Accessing should still work, but setting to an invalid code will panic later
}


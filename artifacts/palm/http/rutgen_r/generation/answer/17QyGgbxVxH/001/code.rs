// Answer 0

#[test]
fn test_response_from_parts() {
    // Define a simple struct to represent Parts
    struct Parts {
        status: StatusCode,
    }
    
    // Define a simple struct for Response
    struct Response<T> {
        head: Parts,
        body: T,
    }
    
    // Define StatusCode enum for demonstration purposes
    #[derive(Debug, PartialEq)]
    enum StatusCode {
        OK,
        BAD_REQUEST,
    }

    // Initialize Parts and Response
    let parts = Parts {
        status: StatusCode::OK,
    };
    let body = "hello world";

    // Create a Response from parts and body
    let response = from_parts(parts, body);
    
    // Ensure the response is created correctly
    assert_eq!(response.head.status, StatusCode::OK);
    assert_eq!(response.body, "hello world");
}

#[test]
fn test_response_from_parts_with_bad_request() {
    // Define a simple struct to represent Parts
    struct Parts {
        status: StatusCode,
    }
    
    // Define a simple struct for Response
    struct Response<T> {
        head: Parts,
        body: T,
    }
    
    // Define StatusCode enum for demonstration purposes
    #[derive(Debug, PartialEq)]
    enum StatusCode {
        OK,
        BAD_REQUEST,
    }

    // Initialize Parts and Response
    let parts = Parts {
        status: StatusCode::BAD_REQUEST,
    };
    let body = "request failed";

    // Create a Response from parts and body
    let response = from_parts(parts, body);
    
    // Ensure the response is created correctly
    assert_eq!(response.head.status, StatusCode::BAD_REQUEST);
    assert_eq!(response.body, "request failed");
}


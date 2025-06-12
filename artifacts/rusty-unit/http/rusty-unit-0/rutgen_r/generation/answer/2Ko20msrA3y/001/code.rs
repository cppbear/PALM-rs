// Answer 0

#[test]
fn test_as_str_valid_range() {
    struct StatusCode(u16);
    
    impl StatusCode {
        pub fn as_str(&self) -> &str {
            let offset = (self.0 - 100) as usize;
            let offset = offset * 3;

            // Invariant: self has checked range [100, 999] and CODE_DIGITS is
            // ASCII-only, of length 900 * 3 = 2700 bytes
            unsafe {
                CODE_DIGITS.get_unchecked(offset..offset + 3)
            }
        }
    }
    
    static CODE_DIGITS: &str = "100OK200OK300OK400OK500OK600OK700OK800OK900OK";

    let status_ok = StatusCode(200);
    assert_eq!(status_ok.as_str(), "200");
    
    let status_not_found = StatusCode(404);
    assert_eq!(status_not_found.as_str(), "404");

    let status_internal_server_error = StatusCode(500);
    assert_eq!(status_internal_server_error.as_str(), "500");
}

#[test]
#[should_panic]
fn test_as_str_below_minimum() {
    struct StatusCode(u16);
    
    impl StatusCode {
        pub fn as_str(&self) -> &str {
            let offset = (self.0 - 100) as usize;
            let offset = offset * 3;

            // Invariant: self has checked range [100, 999] and CODE_DIGITS is
            // ASCII-only, of length 900 * 3 = 2700 bytes
            unsafe {
                CODE_DIGITS.get_unchecked(offset..offset + 3)
            }
        }
    }
    
    static CODE_DIGITS: &str = "100OK200OK300OK400OK500OK600OK700OK800OK900OK";

    let status_below_min = StatusCode(99);
    status_below_min.as_str();
}

#[test]
#[should_panic]
fn test_as_str_above_maximum() {
    struct StatusCode(u16);
    
    impl StatusCode {
        pub fn as_str(&self) -> &str {
            let offset = (self.0 - 100) as usize;
            let offset = offset * 3;

            // Invariant: self has checked range [100, 999] and CODE_DIGITS is
            // ASCII-only, of length 900 * 3 = 2700 bytes
            unsafe {
                CODE_DIGITS.get_unchecked(offset..offset + 3)
            }
        }
    }
    
    static CODE_DIGITS: &str = "100OK200OK300OK400OK500OK600OK700OK800OK900OK";

    let status_above_max = StatusCode(1000);
    status_above_max.as_str();
}


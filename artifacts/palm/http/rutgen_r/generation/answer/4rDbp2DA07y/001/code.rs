// Answer 0

#[test]
fn test_as_u16_ok() {
    struct StatusCode(u16);
    
    impl StatusCode {
        const OK: Self = StatusCode(200);
        
        pub const fn as_u16(&self) -> u16 {
            self.0
        }
    }
    
    let status = StatusCode::OK;
    assert_eq!(status.as_u16(), 200);
}

#[test]
fn test_as_u16_created_status() {
    struct StatusCode(u16);
    
    impl StatusCode {
        pub const fn new(code: u16) -> Self {
            StatusCode(code)
        }
        
        pub const fn as_u16(&self) -> u16 {
            self.0
        }
    }
    
    let status = StatusCode::new(404);
    assert_eq!(status.as_u16(), 404);
}

#[test]
fn test_as_u16_boundary_condition_min() {
    struct StatusCode(u16);
    
    impl StatusCode {
        const MIN: Self = StatusCode(100);
        
        pub const fn as_u16(&self) -> u16 {
            self.0
        }
    }
    
    let status = StatusCode::MIN;
    assert_eq!(status.as_u16(), 100);
}

#[test]
fn test_as_u16_boundary_condition_max() {
    struct StatusCode(u16);
    
    impl StatusCode {
        const MAX: Self = StatusCode(599);
        
        pub const fn as_u16(&self) -> u16 {
            self.0
        }
    }
    
    let status = StatusCode::MAX;
    assert_eq!(status.as_u16(), 599);
}


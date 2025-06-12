// Answer 0

#[cfg(test)]
mod tests {
    use super::*;
    use http::{Request, Extensions};

    #[test]
    fn test_extensions_mut_insert() {
        // Arrange
        let mut request: Request<()> = Request::default();

        // Act
        request.extensions_mut().insert("hello");

        // Assert
        assert_eq!(request.extensions().get::<&str>(), Some(&"hello"));
    }

    #[test]
    fn test_extensions_mut_multiple_inserts() {
        // Arrange
        let mut request: Request<()> = Request::default();
        
        // Act
        request.extensions_mut().insert("hello");
        request.extensions_mut().insert("world");

        // Assert
        assert_eq!(request.extensions().get::<&str>(), Some(&"world"));
    }

    #[test]
    fn test_extensions_mut_initially_empty() {
        // Arrange
        let mut request: Request<()> = Request::default();

        // Act
        let extensions = request.extensions();

        // Assert
        assert!(extensions.get::<&str>().is_none());
    }
}


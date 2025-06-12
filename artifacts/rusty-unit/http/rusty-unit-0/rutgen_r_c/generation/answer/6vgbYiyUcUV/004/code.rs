// Answer 0

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_safe_with_get() {
        let method = Method(Inner::Get);
        assert!(method.is_safe());
    }

    #[test]
    fn test_is_safe_with_head() {
        let method = Method(Inner::Head);
        assert!(method.is_safe());
    }

    #[test]
    fn test_is_safe_with_options() {
        let method = Method(Inner::Options);
        assert!(method.is_safe());
    }

    #[test]
    fn test_is_safe_with_trace() {
        let method = Method(Inner::Trace);
        assert!(method.is_safe());
    }

    #[test]
    fn test_is_safe_with_non_safe_methods() {
        let method_post = Method(Inner::Post);
        let method_put = Method(Inner::Put);
        let method_delete = Method(Inner::Delete);
        let method_patch = Method(Inner::Patch);
        
        assert!(!method_post.is_safe());
        assert!(!method_put.is_safe());
        assert!(!method_delete.is_safe());
        assert!(!method_patch.is_safe());
    }
}


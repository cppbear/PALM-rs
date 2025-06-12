pub fn as_str(&self) -> &str {
        match self.0 {
            Options => "OPTIONS",
            Get => "GET",
            Post => "POST",
            Put => "PUT",
            Delete => "DELETE",
            Head => "HEAD",
            Trace => "TRACE",
            Connect => "CONNECT",
            Patch => "PATCH",
            ExtensionInline(ref inline) => inline.as_str(),
            ExtensionAllocated(ref allocated) => allocated.as_str(),
        }
    }
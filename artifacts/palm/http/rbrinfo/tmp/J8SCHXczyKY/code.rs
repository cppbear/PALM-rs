fn extension_inline(src: &[u8]) -> Result<Method, InvalidMethod> {
        let inline = InlineExtension::new(src)?;

        Ok(Method(ExtensionInline(inline)))
    }
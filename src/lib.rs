/// MetaRust — 一个提供常用辅助工具和函数的 Rust 工具库。
///
/// # 示例
///
/// ```
/// use metarust::greet;
///
/// let message = greet("世界");
/// assert_eq!(message, "Hello, 世界!");
/// ```

/// 返回给定名称的问候消息。
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("World"), "Hello, World!");
        assert_eq!(greet("Rust"), "Hello, Rust!");
    }
}

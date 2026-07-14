/// MetaRust — A Rust utility library providing common helpers and tools.
///
/// # Examples
///
/// ```
/// use metarust::greet;
///
/// let message = greet("World");
/// assert_eq!(message, "Hello, World!");
/// ```

/// Returns a greeting message for the given name.
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

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

use regex::Regex;

/// Replaces all occurrences of `from` with `to` in `input`, supporting both exact string and regex replacements.
///
/// This function works as follows:
/// - If `from` is a **string slice (`&str`)**, it performs a simple `.replace()` on the input.
/// - If `from` is a **regex (`Regex`)**, it applies `.replace_all()` to match the pattern.
/// - The original input string remains **unchanged** and a new `String` is returned.
/// - Consecutive occurrences of `to` are collapsed into a single instance to avoid unintended duplication.
///
/// # Arguments
/// * `input` - The original string.
/// * `from` - The pattern to replace (either a string slice or a regex).
/// * `to` - The replacement string.
///
/// # Returns
/// A new `String` with all occurrences replaced. Consecutive duplicates of `to` are merged.
///
/// # Examples
/// ```
/// use string_replace_all::string_replace_all;
///
/// let text = "Hello world! This is Rust.";
/// let result = string_replace_all(text, "world", "RustLang");
/// assert_eq!(result, "Hello RustLang! This is Rust.");
/// ```
///
/// ```
/// use string_replace_all::string_replace_all;
///
/// let text = "A B C D";
/// let result = string_replace_all(text, "B", "X");
/// assert_eq!(result, "A X C D"); // Spaces are properly collapsed
/// ```
///
/// ```
/// use string_replace_all::string_replace_all;
///
/// let text = "Some special characters like * & % !";
/// let result = string_replace_all(text, "*", "[STAR]");
/// assert_eq!(result, "Some special characters like [STAR] & % !");
/// ```
///
/// ```
/// use regex::Regex;
/// use string_replace_all::string_replace_all;
///
/// let text = "I think Ruth's dog is cuter than your dog!";
/// let regex = Regex::new("(?i)Dog").unwrap(); // Case-insensitive regex
///
/// let result = string_replace_all(text, &regex, "ferret");
/// assert_eq!(result, "I think Ruth's ferret is cuter than your ferret!");
/// ```
pub fn string_replace_all<'a, P: Into<Pattern<'a>>>(
    input: &str,
    pattern: P,
    replacement: &str,
) -> String {
    let mut result = match pattern.into() {
        Pattern::Str(s) => {
            if s == replacement || s.is_empty() {
                return input.to_string();
            }
            input.replace(s, replacement)
        }
        Pattern::Regex(r) => r.replace_all(input, replacement).to_string(),
    };

    if !replacement.is_empty() {
        let cleanup_pattern = Regex::new(&format!("(?:{})+", regex::escape(replacement))).unwrap();
        result = cleanup_pattern
            .replace_all(&result, replacement)
            .to_string();
    }

    result
}

/// Allows both `&str` and `Regex` as input for `from`.
pub enum Pattern<'a> {
    Str(&'a str),
    Regex(Regex),
}

impl<'a> From<&'a str> for Pattern<'a> {
    fn from(s: &'a str) -> Self {
        Pattern::Str(s)
    }
}

impl<'a> From<&'a Regex> for Pattern<'a> {
    fn from(r: &'a Regex) -> Self {
        Pattern::Regex(r.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::string_replace_all;

    #[test]
    fn test_basic_replacement() {
        let input = "Hello world! Hello Rust!";
        let result = string_replace_all(input, "Hello", "Hi");
        assert_eq!(result, "Hi world! Hi Rust!");
    }

    #[test]
    fn test_no_occurrences() {
        let input = "Hello world!";
        let result = string_replace_all(input, "Goodbye", "Hi");
        assert_eq!(result, "Hello world!"); // Should remain unchanged
    }

    #[test]
    fn test_replace_multiple_spaces() {
        let input = "Hello        world!     This      is       Rust.";
        let result = string_replace_all(input, "  ", " "); // Collapse spaces
        assert_eq!(result, "Hello world! This is Rust.");
    }

    #[test]
    fn test_replace_multiple_spaces_doubled() {
        let input = "Hello    world!    This    is    Rust.";
        let result = string_replace_all(input, "    ", "  "); // Collapse to double-spaces
        assert_eq!(result, "Hello  world!  This  is  Rust.");
    }

    #[test]
    fn test_replace_entire_string() {
        let input = "Hello";
        let result = string_replace_all(input, "Hello", "Hi");
        assert_eq!(result, "Hi");
    }

    #[test]
    fn test_replace_with_empty_string() {
        let input = "Hello world!";
        let result = string_replace_all(input, "world!", "");
        assert_eq!(result, "Hello ");
    }

    #[test]
    fn test_replace_empty_string() {
        let input = "Hello world!";
        let result = string_replace_all(input, "", "X");
        assert_eq!(result, "Hello world!"); // Should not change
    }

    #[test]
    fn test_multi_line() {
        let input = r#"Hello (line 1)
        world! (line 2)"#;

        let result = {
            let result = string_replace_all(input, "\n", ""); // Remove newlines
            let result = string_replace_all(&result, "\r", ""); // Remove carriage returns
            let result = string_replace_all(&result, " (line 1)", ""); // Remove labels
            let result = string_replace_all(&result, " (line 2)", "");
            string_replace_all(&result, "  ", " ") // Normalize spaces
        };

        assert_eq!(result, "Hello world!");
    }

    #[test]
    fn test_replace_with_special_characters() {
        let input = "Regex test with $pecial characters!";
        let result = string_replace_all(input, "$pecial", "special");
        assert_eq!(result, "Regex test with special characters!");
    }

    #[test]
    fn test_replace_newlines() {
        let input = "Line1\nLine2\nLine3";
        let result = string_replace_all(input, "\n", " | ");
        assert_eq!(result, "Line1 | Line2 | Line3");
    }

    #[test]
    fn test_replace_unicode() {
        let input = "Привет мир! こんにちは世界!";
        let result = string_replace_all(input, "мир", "Rust");
        assert_eq!(result, "Привет Rust! こんにちは世界!");
    }

    #[test]
    fn test_regex_replacement() {
        let text = "I think Ruth's dog is cuter than your dog!";
        let regex = regex::Regex::new("(?i)Dog").unwrap(); // Case-insensitive regex

        let result = string_replace_all(text, &regex, "ferret");
        assert_eq!(result, "I think Ruth's ferret is cuter than your ferret!");
    }
}

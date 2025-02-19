#[cfg(doctest)]
doc_comment::doctest!("../README.md");

use regex::Regex;

/// A trait that provides a `replace_all` method for `String` and `str` types,
/// enabling both exact string and regex-based replacements.
pub trait StringReplaceAll {
    /// Replaces all occurrences of a pattern with the given replacement.
    ///
    /// This method supports:
    /// - Exact string replacements (`&str`)
    /// - Regular expression-based replacements (`Regex`)
    ///
    /// # Arguments
    /// * `pattern` - The pattern to search for, which can be either:
    ///     - A string slice (`&str`) for simple replacements.
    ///     - A compiled regular expression (`Regex`) for pattern-based replacements.
    /// * `replacement` - The string that will replace occurrences of the pattern.
    ///
    /// # Returns
    /// A new `String` with all occurrences replaced.
    ///
    /// # Examples
    /// ## Using an exact string match
    /// ```
    /// use string_replace_all::StringReplaceAll;
    ///
    /// let text = "I think Ruth's dog is cuter than your dog!";
    /// let result = text.replace_all("dog", "monkey");
    /// assert_eq!(result, "I think Ruth's monkey is cuter than your monkey!");
    /// ```
    ///
    /// ## Using a regular expression match
    /// ```
    /// use regex::Regex;
    /// use string_replace_all::StringReplaceAll;
    ///
    /// let text = "I think Ruth's dog is cuter than your dog!";
    /// let regex = Regex::new("(?i)Dog").unwrap(); // Case-insensitive regex
    ///
    /// let result = text.replace_all(&regex, "ferret");
    /// assert_eq!(result, "I think Ruth's ferret is cuter than your ferret!");
    /// ```
    fn replace_all<'a, P: Into<Pattern<'a>>>(&self, pattern: P, replacement: &str) -> String;
}

/// Implementation of `StringReplaceAll` for `String`.
///
/// This allows direct use of `.replace_all()` on `String` instances.
impl StringReplaceAll for String {
    /// Replaces all occurrences of the given pattern in a `String`.
    ///
    /// # See also
    /// - [`replace_all`](StringReplaceAll::replace_all) for details on arguments and behavior.
    fn replace_all<'a, P: Into<Pattern<'a>>>(&self, pattern: P, replacement: &str) -> String {
        match pattern.into() {
            Pattern::Str(s) => self.replace(s, replacement),
            Pattern::Regex(r) => r.replace_all(self, replacement).to_string(),
        }
    }
}

/// Implementation of `StringReplaceAll` for string slices (`str`).
///
/// This allows direct use of `.replace_all()` on `&str` instances.
impl StringReplaceAll for str {
    /// Replaces all occurrences of the given pattern in a `&str`, returning a `String`.
    ///
    /// This implementation converts the string slice into a `String` and calls
    /// [`replace_all`](StringReplaceAll::replace_all) on it.
    ///
    /// # See also
    /// - [`replace_all`](StringReplaceAll::replace_all) for details on arguments and behavior.
    fn replace_all<'a, P: Into<Pattern<'a>>>(&self, pattern: P, replacement: &str) -> String {
        self.to_string().replace_all(pattern, replacement)
    }
}

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

#[cfg(test)]
mod trait_tests {
    use super::StringReplaceAll;
    use regex::Regex;

    #[test]
    fn test_string_replace_all() {
        let input = "Hello world!".to_string();
        let result = input.replace_all("world", "Rust");

        assert_eq!(result, "Hello Rust!");
    }

    #[test]
    fn test_str_replace_all() {
        let input = "Hello world!";
        let result = input.replace_all("world", "Rust");

        assert_eq!(result, "Hello Rust!");
    }

    #[test]
    fn test_regex_replace_all() {
        let input = "I love RustLang and rust programming!".to_string();
        let regex = Regex::new("(?i)rust").unwrap(); // Case-insensitive

        let result = input.replace_all(&regex, "Go");

        assert_eq!(result, "I love GoLang and Go programming!");
    }

    #[test]
    fn test_replace_special_characters() {
        let input = "Replace * special ** characters!".to_string();
        let result = input.replace_all("*", "-");

        assert_eq!(result, "Replace - special -- characters!");
    }

    #[test]
    fn test_replace_entire_string() {
        let input = "Completely replace this".to_string();
        let result = input.replace_all("Completely replace this", "Done");

        assert_eq!(result, "Done");
    }
}

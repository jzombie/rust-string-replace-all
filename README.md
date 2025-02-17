# string-replace-all

The `string-replace-all` crate provides a utility to return a new `String` with all occurrences of a pattern replaced by a specified replacement. 

- The pattern can be either a **string slice** or a **`Regex`**.
- The replacement is always a **string slice**.
- The original input string remains **unchanged**.

This functionality is inspired by [JavaScript’s `replaceAll()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/replaceAll), with the key difference that **only string slices are supported as replacements** at this time.

## Installation

```bash
cargo add string-replace-all
```

## Usage

```rust
use string_replace_all::string_replace_all;

let text = "I think Ruth's dog is cuter than your dog!";

let result = string_replace_all(text, "dog", "monkey");
assert_eq!(result, "I think Ruth's monkey is cuter than your monkey!");
```

```rust
use regex::Regex;
use string_replace_all::string_replace_all;

let text = "I think Ruth's dog is cuter than your dog!";
let regex = Regex::new("(?i)Dog").unwrap(); // Case-insensitive regex

let result = string_replace_all(text, &regex, "ferret");
assert_eq!(result, "I think Ruth's ferret is cuter than your ferret!");
```

## Testing

Run tests with:
```sh
cargo test
```

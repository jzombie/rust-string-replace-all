# string-replace-all

[![made-with-rust][rust-logo]][rust-src-page]
[![crates.io][crates-badge]][crates-page]
[![Documentation][docs-badge]][docs-page]
[![MIT licensed][license-badge]][license-page]

| OS            | Status                                                                               |
|---------------|--------------------------------------------------------------------------------------|
| Ubuntu-latest | [![Ubuntu Tests][ubuntu-latest-badge]][ubuntu-latest-workflow]                       |
| macOS-latest  | [![macOS Tests][macos-latest-badge]][macos-latest-workflow]                          |
| Windows-latest| [![Windows Tests][windows-latest-badge]][windows-latest-workflow]                    |

The `string-replace-all` crate enables `JavaScript-style` string replacement, returning a new `String` where all occurrences of a pattern are substituted with a specified replacement. It supports both exact matches and regex-based replacements.

- The pattern can be either a **string slice** or a **`Regex`**.
- The replacement is always a **string slice**.
- The original input string remains **unchanged**.

This functionality is inspired by [JavaScript’s `replaceAll()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/replaceAll), with the key difference that **only string slices are supported as replacements** at this time.

## Install

```bash
cargo add string-replace-all
```

## Usage

### Example 1: Using `StringReplaceAll` Trait

The `StringReplaceAll` trait extends String and string slices with a `replace_all` method, allowing for both exact string and regex-based replacements.

```rust
use string_replace_all::StringReplaceAll;

let text = "I think Ruth's dog is cuter than your dog!";
let result = text.replace_all("dog", "monkey");

assert_eq!(result, "I think Ruth's monkey is cuter than your monkey!");
```

```rust
use regex::Regex;
use string_replace_all::StringReplaceAll;

let text = "I think Ruth's dog is cuter than your dog!";
let regex = Regex::new("(?i)Dog").unwrap(); // Case-insensitive regex

let result = text.replace_all(&regex, "ferret");

assert_eq!(result, "I think Ruth's ferret is cuter than your ferret!");
```

### Example 2: Using `string_replace_all` Function

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

## License

[MIT License](LICENSE) (c) 2025 Jeremy Harris.

[rust-src-page]: https://www.rust-lang.org/
[rust-logo]: https://img.shields.io/badge/Made%20with-Rust-black?&logo=Rust

[crates-page]: https://crates.io/crates/string-replace-all
[crates-badge]: https://img.shields.io/crates/v/string-replace-all.svg

[docs-page]: https://docs.rs/string-replace-all
[docs-badge]: https://docs.rs/string-replace-all/badge.svg

[license-page]: https://github.com/jzombie/rust-string-replace-all/blob/main/LICENSE
[license-badge]: https://img.shields.io/badge/license-MIT-blue.svg

[ubuntu-latest-badge]: https://github.com/jzombie/rust-string-replace-all/actions/workflows/rust-tests.yml/badge.svg?branch=main&job=Run%20Rust%20Tests%20(OS%20=%20ubuntu-latest)
[ubuntu-latest-workflow]: https://github.com/jzombie/rust-string-replace-all/actions/workflows/rust-tests.yml?query=branch%3Amain

[macos-latest-badge]: https://github.com/jzombie/rust-string-replace-all/actions/workflows/rust-tests.yml/badge.svg?branch=main&job=Run%20Rust%20Tests%20(OS%20=%20macos-latest)
[macos-latest-workflow]: https://github.com/jzombie/rust-string-replace-all/actions/workflows/rust-tests.yml?query=branch%3Amain

[windows-latest-badge]: https://github.com/jzombie/rust-string-replace-all/actions/workflows/rust-tests.yml/badge.svg?branch=main&job=Run%20Rust%20Tests%20(OS%20=%20windows-latest)
[windows-latest-workflow]: https://github.com/jzombie/rust-string-replace-all/actions/workflows/rust-tests.yml?query=branch%3Amain

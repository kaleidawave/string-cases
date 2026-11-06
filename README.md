# String cases

[![crates.io badge](https://img.shields.io/crates/v/string-cases?style=flat-square)](https://crates.io/crates/string-cases)
[![docs.rs badge](https://img.shields.io/docsrs/string-cases?style=flat-square)](https://docs.rs/string-cases/latest)
[![compiler-club](https://f005.backblazeb2.com/file/6948446f806c172a098591088e0715b8258b380b/join-compiler-club.svg)](https://kaleidawave-public-pages.val.run/r/join-cc)

With the `StringCasesExt` trait, provides methods for getting snake, kebab and pascal case output.

```rust
use string_cases::StringCasesExt;

let input = "HelloWorld";
let output: std::borrow::Cow<str> = input.to_snake_case();

assert_eq!(output, "hello_world");

// other cases
assert_eq!(input.to_kebab_case(), "hello-world");
assert_eq!("hello world".to_camel_case(), "helloWorld");
assert_eq!("hello world".to_pascal_case(), "HelloWorld");
```

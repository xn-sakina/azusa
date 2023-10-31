# azusa

Transform string index from `UTF-8` in Rust to `UTF-16` in JavsScript.

### Usage

```rust
use azusa;

let text = "c😅é文";
let transformer = Azusa::new(text.into());

let utf8_range_in_rust = (1, 5);
let utf16_range_in_js = (1, 3);
assert_eq!(
    transformer.utf8_to_utf16(utf8_range_in_rust),
    utf16_range_in_js
);

// In javascript: text.slice(1, 3) === "😅"
```

### License

MIT

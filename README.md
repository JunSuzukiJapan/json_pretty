# json_pretty

Simple Json prettifier for Rust language.

# Installation

In Cargo.toml:

```
[dependencies]
json_pretty = "*"
```

# Example

```
extern crate json_pretty;
use json_pretty::PrettyFormatter;

// ...

let s = r#"{"description": "string for test", "id" : 123, "true" : true}"#;
let formatter = PrettyFormatter::from_str(s);
let result = formatter.pretty();
println!("s: {}", result);
```

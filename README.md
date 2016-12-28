# json_pretty

Json prettifier for Rust language.

# Example

```
use json_pretty::PrettyFormatter;
```

```
let s = r#"{"description": "string for test", "id" : 123, "true" : true }"#;
let formatter = PrettyFormatter::from_str(s);
let result = formatter.pretty();
println!("s: {}", result);
```

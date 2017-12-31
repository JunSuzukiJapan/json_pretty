use std::fmt;

#[derive(Debug)]
pub struct PrettyFormatter {
    string: String,
    indent: i32
}

impl PrettyFormatter {
   pub fn from_str(s: &str) -> PrettyFormatter {
        PrettyFormatter {
            string: s.to_owned(),
            indent: 2
        }
    }

    pub fn from_string(s: &String) -> PrettyFormatter {
        PrettyFormatter {
            string: s.clone(),
            indent: 2
        }
    }

    pub fn indent(&mut self, indent: i32) -> PrettyFormatter {
        PrettyFormatter {
            string: self.string.clone(),
            indent: indent,
        }
    }

    pub fn pretty(&self) -> String {
        let mut result = String::new();

        let mut in_string = false;
        let mut indent = 0;
        let mut need_indent = false;

        for ch in self.string.chars() {
            match ch {
                '{' => {
                    if need_indent {
                        for _ in 0..indent {
                            result.push(' ');
                        }
                        need_indent = false;
                    }
                    result.push('{');
                    if !in_string {
                        indent += self.indent;
                        result.push('\n');
                        need_indent = true;
                    }
                },
                '}' => {
                    if !in_string {
                        result.push('\n');
                        indent -= self.indent;
                        for _ in 0..indent {
                            result.push(' ');
                        }
                    }
                    result.push('}');
                },
                '"' => {
                    if need_indent {
                        for _ in 0..indent {
                            result.push(' ');
                        }
                    }
                    result.push('"');
                    in_string = !in_string;
                    need_indent = false;
                },
                ',' => {
                    //println!("',' found'");
                    if need_indent {
                        for _ in 0..indent {
                            result.push(' ');
                        }
                        need_indent = false;
                    }
                    result.push(',');
                    if !in_string {
                        result.push('\n');
                        need_indent = true;
                    }
                },
                ch @ ' ' | ch @ '\t' => {
                    if in_string {
                        result.push(ch);
                    }else{
                        if need_indent {
                            continue;
                        }else{
                            result.push(ch);
                        }
                    }
                },
                '\n' => {
                    if in_string {
                        result.push('\n');
                    }else{
                        need_indent = true;
                        continue;
                    }
                }
                c => {
                    if need_indent {
                        for _ in 0..indent {
                            result.push(' ');
                        }
                    }
                    need_indent = false;
                    result.push(c);
                },
            }
        }

        result
    }
}

impl fmt::Display for PrettyFormatter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.pretty())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_pretty_formatter() {
        use super::PrettyFormatter;

        let s = r#"{"description": "string for test", "id" : 123, "true" : true, "nest": { "foo" : "bar"}}"#;
        let formatter = PrettyFormatter::from_str(s);
        let result = formatter.pretty();
        println!("s: {}", result);
        assert_eq!(r#"{
  "description": "string for test",
  "id" : 123,
  "true" : true,
  "nest": {
    "foo" : "bar"
  }
}"#, result);
    }
}

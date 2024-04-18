#[derive(Default)]
struct Args {
    end_without_newline_character: bool,
    interpret_escape: bool,
    inner: Vec<String>,
}

impl Args {
    pub fn parse_args(args: Vec<String>) -> Args {
        let mut inner: Vec<String> = vec![];
        let mut end_without_newline_character = false;
        let mut interpret_escape = false;

        for arg in args.into_iter().skip(1) {
            match arg.clone().as_str() {
                "-n" => end_without_newline_character = true,
                "-e" => interpret_escape = true,
                "-E" => interpret_escape = false,
                _ => inner.push(arg),
            }
        }

        Args {
            end_without_newline_character,
            interpret_escape,
            inner,
        }
    }

    pub fn new() -> Args {
        Self::parse_args(std::env::args().collect())
    }

    fn apply_escape_characters(input: String) -> String {
        let mut output = "".to_string();
        let mut escape_flag = false;
        for character in input.chars() {
            if escape_flag {
                escape_flag = false;
                match character {
                    'n' => output.push('\n'),
                    't' => output.push('\t'),
                    '\\' => output.push('\\'),
                    _ => {
                        output.push('\\');
                        output.push(character);
                    }
                }
                continue;
            }
            if character == '\\' {
                escape_flag = true;
                continue;
            }
            output.push(character);
        }
        output
    }

    pub fn echo(self) -> String {
        let mut output = self.inner.into_iter().collect::<Vec<String>>().join(" ");
        if !self.end_without_newline_character {
            output += "\n";
        }
        if self.interpret_escape {
            output = Self::apply_escape_characters(output);
        }
        output
    }
}

fn main() {
    let args = Args::new();
    print!("{}", args.echo());
}

#[cfg(test)]
mod parse_args {
    use super::*;

    #[test]
    fn parsing_without_args() {
        let string_args = vec!["file_name".to_string()];
        let parsed_args = Args::parse_args(string_args);
        assert_eq!(parsed_args.inner.len(), 0);
    }

    #[test]
    fn parsing_args_without_flags() {
        let test_arg = "Hello, World!";
        let string_args = vec!["file_name".to_string(), test_arg.to_string()];
        let parsed_args = Args::parse_args(string_args);
        assert_eq!(parsed_args.inner.len(), 1);
        assert_eq!(parsed_args.inner.first().unwrap().as_str(), test_arg);
    }

    #[test]
    fn parsing_no_newline_flag() {
        let string_args = vec!["file_name".to_string(), "-n".to_string()];
        let parsed_args = Args::parse_args(string_args);
        assert_eq!(parsed_args.inner.len(), 0);
        assert!(parsed_args.end_without_newline_character);
    }

    #[test]
    fn parsing_interpret_escape_flag() {
        let string_args = vec!["file_name".to_string(), "-e".to_string()];
        let parsed_args = Args::parse_args(string_args);
        assert_eq!(parsed_args.inner.len(), 0);
        assert!(parsed_args.interpret_escape);
    }

    #[test]
    fn parsing_do_not_interpret_escape_flag() {
        let string_args = vec!["file_name".to_string(), "-E".to_string()];
        let parsed_args = Args::parse_args(string_args);
        assert_eq!(parsed_args.inner.len(), 0);
        assert!(!parsed_args.interpret_escape);
    }
}

#[cfg(test)]
mod echo {
    use super::*;

    #[test]
    fn echo_no_args() {
        let string_args = vec!["file_name".to_string()];
        let args = Args::parse_args(string_args);
        assert_eq!(args.echo().as_str(), "\n");
    }

    #[test]
    fn echo_simple_text() {
        let test_arg = "Hello, World!".to_string();
        let string_args = vec!["file_name".to_string(), test_arg.clone()];
        let args = Args::parse_args(string_args);
        assert_eq!(args.echo(), test_arg + "\n");
    }

    #[test]
    fn echo_multiple_args() {
        let test_arg1 = "one".to_string();
        let test_arg2 = "two".to_string();
        let string_args = vec![
            "file_name".to_string(),
            test_arg1.clone(),
            test_arg2.clone(),
        ];
        let args = Args::parse_args(string_args);
        assert_eq!(args.echo().as_str(), test_arg1 + " " + &test_arg2 + "\n");
    }

    #[test]
    fn echo_with_no_ending_newline_character() {
        let test_arg = "Hello, World!".to_string();
        let string_args = vec!["file_name".to_string(), "-n".to_string(), test_arg.clone()];
        let args = Args::parse_args(string_args);
        assert_eq!(args.echo(), test_arg);
    }
}

#[cfg(test)]
mod escape_characters {
    use super::*;

    #[test]
    fn no_escape_character() {
        let input = "Hello, World!".to_string();
        let output = Args::apply_escape_characters(input);
        assert_eq!(output, "Hello, World!");
    }

    #[test]
    fn newline_escape_character() {
        let input = "Hello, World!\\n".to_string();
        let output = Args::apply_escape_characters(input);
        assert_eq!(output, "Hello, World!\n");
    }

    #[test]
    fn backslash_escape_character() {
        let input = "Hello, World!\\\\".to_string();
        let output = Args::apply_escape_characters(input);
        assert_eq!(output, "Hello, World!\\");
    }

    #[test]
    fn tab_escape_character() {
        let input = "Hello, World!\\t".to_string();
        let output = Args::apply_escape_characters(input);
        assert_eq!(output, "Hello, World!\t");
    }

    #[test]
    fn multiple_escape_characters() {
        let input = "Hello, \\tWorld!\\n".to_string();
        let output = Args::apply_escape_characters(input);
        assert_eq!(output, "Hello, \tWorld!\n");
    }

    #[test]
    fn invalid_escape_character() {
        let input = "Hello, World!\\_".to_string();
        let output = Args::apply_escape_characters(input);
        assert_eq!(output, "Hello, World!\\_");
    }
}

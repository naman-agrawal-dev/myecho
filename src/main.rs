#[derive(Default)]
struct Args {
    end_without_newline_character: bool,
    interpret_escape: bool,
    inner: Vec<String>,
}

impl Args {
    pub fn parse_args(args: Vec<String>) -> Args {
        Args {
            end_without_newline_character: false,
            interpret_escape: false,
            inner: args.into_iter().skip(1).collect(),
        }
    }

    pub fn new() -> Args {
        Self::parse_args(std::env::args().collect())
    }

    pub fn echo(self) -> String {
        self.inner.into_iter().collect::<String>().to_string() + "\n"
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
}

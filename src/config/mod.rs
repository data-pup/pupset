use std::io::{stdin, Stdin};
// This trait may be needed once file inputs are allowed.
// use std::io::BufRead;

/// Config struct represents the program configuration and
/// command arguments for the command pipeline.
pub struct Config {
    pub argv: Vec<String>,
    input: Stdin,
}

impl Config {
    pub fn new(argv: Vec<String>) -> Self {
        Self {
            argv,
            input: stdin(),
        }
    }
}

#[cfg(test)]
mod tests {
    use config::*;

    #[test]
    fn it_works() {
        let args: Vec<String> = vec![
            "hello".to_owned(),
            "world".to_owned()
        ];
        let c = Config::new(args);
        assert_eq!(c.argv, vec!["hello", "world"]);
    }
}

use command::Command;
use command::address_condition::parse_arg;

fn parse_command(args: Vec<String>) -> Command {
    // TODO:
    // 1. Check if address condition was given
    // 2. Resolve command name, corresponding requirements.
    // 3. Parse individual strings, form command object.
    unimplemented!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn placeholder() {
        assert_eq!(1, 1);
    }
}
